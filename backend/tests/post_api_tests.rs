use anyhow::{Context, Result};
use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode},
};
use backend::utils::hash_password;
use backend::{
    config::{AppConfig, AuthConfig, DatabaseConfig, DraftPolicy, EmailConfig, ServerConfig},
    dtos::{
        PaginatedResponse,
        post::{CreatePostPayload, PostDetailDto, UpdatePostPayload},
    },
    handlers::AppState,
    models::{Category, Post, Role, Tag, User},
    repositories::{
        CategoryRepository, LoginAttemptRepository, OneTimeTokenRepository, PermissionRepository,
        PostRepository, PostgresCategoryRepository, PostgresLoginAttemptRepository,
        PostgresOneTimeTokenRepository, PostgresPermissionRepository, PostgresPostRepository,
        PostgresRoleRepository, PostgresTagRepository, PostgresUserRepository, RoleRepository,
        TagRepository, UserRepository,
    },
    routes::create_router,
    services::{
        AdminService, AuthService, CategoryService, EmailService, PostService, TagService,
        UserService,
    },
};
use http_body_util::BodyExt;
use slug::slugify;
use sqlx::PgPool;
use std::sync::{Arc, Once};
use tower::ServiceExt;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

// --- 日志初始化 ---
static TRACING_INIT_TEST: Once = Once::new();

fn ensure_tracing_is_initialized_for_test() {
    TRACING_INIT_TEST.call_once(|| {
        let default_filter = "info,backend=trace,sqlx=warn";
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(default_filter));
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_test_writer()
            .try_init()
            .ok();
    });
}

// --- 测试环境设置函数 ---
async fn setup_test_app(pool: PgPool) -> Router {
    ensure_tracing_is_initialized_for_test();

    // 1. 为测试手动创建完整的 AppConfig
    let test_config = AppConfig {
        database: DatabaseConfig {
            url: String::new(), // 未使用, 因为 pool 是直接注入的
        },
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        auth: AuthConfig {
            jwt_secret: "test_secret_for_posts".to_string(),
            jwt_issuer: "test_issuer".to_string(),
            jwt_audience: "test_audience".to_string(),
            access_token_expiry_minutes: 5,
            refresh_token_expiry_days: 1,
            max_login_failures: 5,
            lockout_duration_seconds: 900,
        },
        email: EmailConfig {
            smtp_host: "localhost".to_string(),
            smtp_port: 1025, // 本地 SMTP 的默认端口
            smtp_user: "".to_string(),
            smtp_pass: "".to_string(),
            from_address: "test@example.com".to_string(),
        },
        draft_policy: DraftPolicy {
            mode: "private".to_string(),
            admin_access_all_drafts: false,
            audit_draft_access: true,
        },
    };

    // 2. 实例化所有 Repositories
    let user_repo: Arc<dyn UserRepository> = Arc::new(PostgresUserRepository::new(pool.clone()));
    let role_repo: Arc<dyn RoleRepository> = Arc::new(PostgresRoleRepository::new(pool.clone()));
    let permission_repo: Arc<dyn PermissionRepository> =
        Arc::new(PostgresPermissionRepository::new(pool.clone()));
    let one_time_token_repo: Arc<dyn OneTimeTokenRepository> =
        Arc::new(PostgresOneTimeTokenRepository::new(pool.clone()));
    let login_attempt_repo: Arc<dyn LoginAttemptRepository> =
        Arc::new(PostgresLoginAttemptRepository::new(pool.clone()));
    let category_repo: Arc<dyn CategoryRepository> =
        Arc::new(PostgresCategoryRepository::new(pool.clone()));
    let tag_repo: Arc<dyn TagRepository> = Arc::new(PostgresTagRepository::new(pool.clone()));
    let post_repo: Arc<dyn PostRepository> = Arc::new(PostgresPostRepository::new(pool.clone()));

    // 3. 实例化所有 Services
    let email_service = Arc::new(EmailService::new(test_config.email.clone()));
    let auth_service = Arc::new(AuthService::new(
        user_repo.clone(),
        role_repo.clone(),
        login_attempt_repo,
        one_time_token_repo,
        email_service,
        &test_config,
    ));
    let admin_service = Arc::new(AdminService::new(
        user_repo.clone(),
        role_repo.clone(),
        permission_repo,
    ));
    let user_service = Arc::new(UserService::new(user_repo.clone()));
    let category_service = Arc::new(CategoryService::new(category_repo.clone()));
    let tag_service = Arc::new(TagService::new(tag_repo.clone()));
    let post_service = Arc::new(PostService::new(
        post_repo.clone(),
        category_repo.clone(),
        tag_repo.clone(),
        user_repo.clone(),
    ));

    // 4. 创建完整的 AppState
    let app_state = AppState {
        post_service,
        category_service,
        tag_service,
        auth_service,
        admin_service,
        user_service,
    };

    // 5. 创建 Router
    create_router(app_state)
}

// --- 测试辅助函数 ---

/// 注册一个新用户，并赋予指定角色
async fn seed_user_with_role(pool: &PgPool, name: &str, role_name: &str) -> Result<User> {
    let role = sqlx::query_as!(
        Role,
        "SELECT id, name, description, created_at, updated_at FROM roles WHERE name = $1",
        role_name
    )
    .fetch_optional(pool)
    .await?
    .context(format!(
        "Role '{}' not found in database. Make sure migrations are run.",
        role_name
    ))?;

    let valid_password_hash = hash_password("StrongPassword123!")?;

    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (id, username, email, hashed_password, email_verified_at) VALUES ($1, $2, $3, $4, NOW())
           RETURNING id, username, email, hashed_password, created_at, updated_at, email_verified_at"#,
        Uuid::new_v4(),
        name,
        format!("{}@example.com", name),
        valid_password_hash
    )
        .fetch_one(pool)
        .await?;

    sqlx::query!(
        "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)",
        user.id,
        role.id
    )
    .execute(pool)
    .await?;

    Ok(user)
}

/// 辅助函数：通过模拟API请求为指定用户登录并获取token
async fn get_token_for_user(app: &Router, username: &str, password: &str) -> Result<String> {
    let login_payload = serde_json::json!({ "username": username, "password": password });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&login_payload)?))?,
        )
        .await?;

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes)?;

    let token = body_json["access_token"]
        .as_str()
        .context("access_token not found in login response")?
        .to_string();

    Ok(token)
}

/// 辅助函数：注册一个随机用户，并以该用户身份登录，返回(token, user_id)
async fn register_and_login_new_user(app: &Router) -> Result<(String, Uuid)> {
    let username = format!("testuser_{}", Uuid::new_v4());
    let email = format!("test_{}@example.com", username);
    let password = "StrongPassword123!";

    let register_payload = serde_json::json!({
        "username": &username,
        "email": &email,
        "password": &password,
        "confirm_password": &password
    });
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&register_payload)?))?,
        )
        .await?;

    let login_payload = serde_json::json!({ "username": &username, "password": &password });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&login_payload)?))?,
        )
        .await?;

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let login_response: serde_json::Value = serde_json::from_slice(&body_bytes)?;

    let token = login_response["access_token"]
        .as_str()
        .context("Login response did not contain 'access_token'")?
        .to_string();

    let user_id_str = login_response["user"]["id"]
        .as_str()
        .context("Login response did not contain 'user.id'")?;

    let user_id = Uuid::parse_str(user_id_str)?;

    Ok((token, user_id))
}

/// 辅助函数：在数据库中插入一篇文章
async fn seed_one_post(
    pool: &PgPool,
    author_id: Uuid,
    title: &str,
    content: &str, // 接受 Markdown 内容
    published: bool,
) -> Result<Post> {
    let slug = slugify(title);
    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (id, slug, title, content, author_id, published_at, draft_shared_with, is_draft_public)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, slug, title, content, author_id, created_at, updated_at, published_at, draft_shared_with, is_draft_public
        "#,
        Uuid::new_v4(),
        slug,
        title,
        content, // 存储 Markdown 内容
        author_id,
        if published {
            Some(chrono::Utc::now())
        } else {
            None
        },
        None::<&[Uuid]>, // draft_shared_with
        Some(false)      // is_draft_public
    )
    .fetch_one(pool)
    .await?;
    Ok(post)
}

/// 辅助函数：创建分类和标签
async fn seed_one_category(pool: &PgPool, name: &str) -> Result<Category> {
    sqlx::query_as!(
        Category,
        r#"INSERT INTO categories (id, name, slug) VALUES ($1, $2, $3) RETURNING *;"#,
        Uuid::new_v4(),
        name,
        slugify(name)
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

async fn seed_one_tag(pool: &PgPool, name: &str) -> Result<Tag> {
    sqlx::query_as!(
        Tag,
        r#"INSERT INTO tags (id, name, slug) VALUES ($1, $2, $3) RETURNING *;"#,
        Uuid::new_v4(),
        name,
        slugify(name)
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

// --- 帖子API集成测试 ---

// == 创建帖子 (POST /posts)

#[sqlx::test]
async fn test_create_post_as_user_success(pool: PgPool) -> Result<()> {
    // 准备
    let app = setup_test_app(pool.clone()).await;
    let (token, user_id) = register_and_login_new_user(&app).await?;

    let payload = CreatePostPayload {
        title: "一个由普通用户创建的帖子".to_string(),
        content: "这是帖子的**Markdown**内容...".to_string(),
        category_ids: None,
        tag_ids: None,
        draft_shared_with: None,
        is_draft_public: None,
    };

    // 执行
    let request = Request::builder()
        .method(Method::POST)
        .uri("/posts")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_vec(&payload)?))?;
    let response = app.oneshot(request).await?;

    // 断言
    assert_eq!(response.status(), StatusCode::CREATED);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let created_post: PostDetailDto = serde_json::from_slice(&body_bytes)?;

    assert_eq!(created_post.title, payload.title);
    assert_eq!(created_post.content_markdown, payload.content);
    assert_eq!(
        created_post.content_html,
        "<p>这是帖子的<strong>Markdown</strong>内容...</p>\n"
    );
    assert_eq!(created_post.author_id, Some(user_id));

    // 从数据库验证作者ID是否正确
    let db_post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", created_post.id)
        .fetch_one(&pool)
        .await?;
    assert_eq!(db_post.author_id, Some(user_id));

    Ok(())
}

#[sqlx::test]
async fn test_create_post_with_associations_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let (token, _user_id) = register_and_login_new_user(&app).await?;
    let category = seed_one_category(&pool, "关联分类").await?;
    let tag = seed_one_tag(&pool, "关联标签").await?;

    let payload = CreatePostPayload {
        title: "带有关联的帖子".to_string(),
        content: "内容...".to_string(),
        category_ids: Some(vec![category.id]),
        tag_ids: Some(vec![tag.id]),
        draft_shared_with: None,
        is_draft_public: None,
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/posts")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_vec(&payload)?))?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let created_post: PostDetailDto = serde_json::from_slice(&body_bytes)?;

    assert!(created_post.categories.is_some());
    assert_eq!(created_post.categories.as_ref().unwrap().len(), 1);
    assert_eq!(created_post.categories.as_ref().unwrap()[0].id, category.id);
    assert!(created_post.tags.is_some());
    assert_eq!(created_post.tags.as_ref().unwrap()[0].id, tag.id);

    Ok(())
}

#[sqlx::test]
async fn test_create_post_no_token_fails(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;
    let payload = CreatePostPayload {
        title: "无Token帖子".into(),
        content: "...".into(),
        category_ids: None,
        tag_ids: None,
        draft_shared_with: None,
        is_draft_public: None,
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/posts")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

// == 读取帖子 (GET /posts, GET /posts/{id})

#[sqlx::test]
async fn test_list_posts_pagination(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let (token, user_id) = register_and_login_new_user(&app).await?;
    for i in 0..15 {
        seed_one_post(&pool, user_id, &format!("Post {}", i), "list content", true).await?;
    }

    let request = Request::builder()
        .uri("/posts?page=2&page_size=5")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let page: PaginatedResponse<PostDetailDto> = serde_json::from_slice(&body_bytes)?;

    assert_eq!(page.total_items, 15);
    assert_eq!(page.page, 2);
    assert_eq!(page.page_size, 5);
    assert_eq!(page.total_pages, 3);
    assert_eq!(page.items.len(), 5);
    // 验证列表中的内容也被正确转换
    assert!(page.items[0].content_html.contains("<p>list content</p>"));

    Ok(())
}

#[sqlx::test]
async fn test_get_post_by_slug_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let (token, user_id) = register_and_login_new_user(&app).await?;
    let markdown_content = "# Slug测试\n\n- item 1\n- item 2";
    let expected_html = "<h1>Slug测试</h1>\n<ul>\n<li>item 1</li>\n<li>item 2</li>\n</ul>\n";
    let seeded_post = seed_one_post(
        &pool,
        user_id,
        "一个用于Slug测试的帖子",
        markdown_content,
        true,
    )
    .await?;

    let request = Request::builder()
        .uri(format!("/posts/{}", seeded_post.slug))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched_post: PostDetailDto = serde_json::from_slice(&body_bytes)?;

    assert_eq!(fetched_post.id, seeded_post.id);
    assert_eq!(fetched_post.content_markdown, markdown_content);
    assert_eq!(fetched_post.content_html, expected_html);

    Ok(())
}

// == 更新帖子 (PUT /posts/{id})

#[sqlx::test]
async fn test_update_own_post_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let (token, user_id) = register_and_login_new_user(&app).await?;
    let original_post = seed_one_post(&pool, user_id, "我的原始帖子", "原始内容", false).await?;

    let payload = UpdatePostPayload {
        title: Some("我的更新后的帖子标题".to_string()),
        content: Some("`更新后`的内容".to_string()),
        ..Default::default()
    };

    let request = Request::builder()
        .method(Method::PUT)
        .uri(format!("/posts/{}", original_post.id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_vec(&payload)?))?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let updated_post: PostDetailDto = serde_json::from_slice(&body_bytes)?;

    assert_eq!(updated_post.title, payload.title.clone().unwrap());
    assert_eq!(
        updated_post.content_markdown,
        payload.content.clone().unwrap()
    );
    assert_eq!(
        updated_post.content_html,
        "<p><code>更新后</code>的内容</p>\n"
    );

    Ok(())
}

#[sqlx::test]
async fn test_update_post_not_owner_fails(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let (_owner_token, owner_id) = register_and_login_new_user(&app).await?;
    let post_to_update =
        seed_one_post(&pool, owner_id, "一篇不让别人改的帖子", "content", true).await?;
    let (attacker_token, _attacker_id) = register_and_login_new_user(&app).await?;

    let payload = UpdatePostPayload {
        title: Some("恶意修改".to_string()),
        ..Default::default()
    };

    let request = Request::builder()
        .method(Method::PUT)
        .uri(format!("/posts/{}", post_to_update.id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", attacker_token))
        .body(Body::from(serde_json::to_vec(&payload)?))?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    Ok(())
}

#[sqlx::test]
async fn test_update_post_as_editor_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let (_owner_token, owner_id) = register_and_login_new_user(&app).await?;
    let post_to_update =
        seed_one_post(&pool, owner_id, "一篇编辑可以改的帖子", "content", true).await?;
    let editor = seed_user_with_role(&pool, "editor_user_for_post", "editor").await?;
    let editor_token = get_token_for_user(&app, &editor.username, "StrongPassword123!").await?;

    let payload = UpdatePostPayload {
        title: Some("由编辑修改".to_string()),
        ..Default::default()
    };

    let request = Request::builder()
        .method(Method::PUT)
        .uri(format!("/posts/{}", post_to_update.id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", editor_token))
        .body(Body::from(serde_json::to_vec(&payload)?))?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

// == 删除帖子 (DELETE /posts/{id})

#[sqlx::test]
async fn test_delete_own_post_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let (token, user_id) = register_and_login_new_user(&app).await?;
    let post_to_delete = seed_one_post(&pool, user_id, "一篇待删除的帖子", "c", true).await?;

    let request = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/posts/{}", post_to_delete.id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM posts WHERE id = $1",
        post_to_delete.id
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(count.unwrap_or(0), 0);

    Ok(())
}

#[sqlx::test]
async fn test_delete_post_not_owner_fails(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let (_owner_token, owner_id) = register_and_login_new_user(&app).await?;
    let post_to_delete = seed_one_post(&pool, owner_id, "不让他人删除的帖子", "c", true).await?;
    let (attacker_token, _attacker_id) = register_and_login_new_user(&app).await?;

    let request = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/posts/{}", post_to_delete.id))
        .header("Authorization", format!("Bearer {}", attacker_token))
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    Ok(())
}
