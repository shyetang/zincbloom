use anyhow::{Context, Result};
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    Router,
};
use backend::{
    config::{AppConfig, AuthConfig, DatabaseConfig, ServerConfig},
    dtos::{post::PostDetailDto, CreatePostPayload, PaginatedResponse, UpdatePostPayload},
    handlers::AppState,
    models::{Category, Post, Tag},
    repositories::{
        CategoryRepository, PermissionRepository, PostRepository, PostgresCategoryRepository,
        PostgresPermissionRepository, PostgresPostRepository, PostgresRoleRepository,
        PostgresTagRepository, PostgresUserRepository, RoleRepository, TagRepository,
        UserRepository,
    },
    routes::create_router,
    services::{AuthSerVice, CategoryService, PostService, TagService},
};
use chrono::{DateTime, Utc};
use http_body_util::BodyExt;
use slug::slugify;
use sqlx::PgPool;
use std::sync::{Arc, Once};
use tower::ServiceExt;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

// --- 日志初始化 (保持不变) ---
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

// --- 统一的测试环境设置函数 ---
async fn setup_test_app(pool: PgPool) -> Router {
    ensure_tracing_is_initialized_for_test();

    // 1. 为测试手动创建 AppConfig
    let test_config = AppConfig {
        database: DatabaseConfig { url: String::new() }, // 未使用
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        auth: AuthConfig {
            jwt_secret: "test_secret".to_string(),
            jwt_issuer: "test_issuer".to_string(),
            jwt_audience: "test_audience".to_string(),
            jwt_expiry_hours: 1,
        },
    };

    // 2. 实例化所有 Repositories
    let category_repo: Arc<dyn CategoryRepository> =
        Arc::new(PostgresCategoryRepository::new(pool.clone()));
    let tag_repo: Arc<dyn TagRepository> = Arc::new(PostgresTagRepository::new(pool.clone()));
    let post_repo: Arc<dyn PostRepository> = Arc::new(PostgresPostRepository::new(pool.clone()));
    let user_repo: Arc<dyn UserRepository> = Arc::new(PostgresUserRepository::new(pool.clone()));
    let role_repo: Arc<dyn RoleRepository> = Arc::new(PostgresRoleRepository::new(pool.clone()));
    let _permission_repo: Arc<dyn PermissionRepository> =
        Arc::new(PostgresPermissionRepository::new(pool.clone()));

    // 3. 实例化所有 Services
    let auth_service = Arc::new(AuthSerVice::new(
        user_repo.clone(),
        role_repo.clone(),
        &test_config,
    ));
    let category_service = Arc::new(CategoryService::new(category_repo.clone()));
    let tag_service = Arc::new(TagService::new(tag_repo.clone()));
    let post_service = Arc::new(PostService::new(
        post_repo.clone(),
        category_repo.clone(),
        tag_repo.clone(),
    ));

    // 4. 创建更新后的 AppState
    let app_state = AppState {
        post_service,
        category_service,
        tag_service,
        auth_service,
    };

    // 5. 创建 Router
    create_router(app_state)
}

// --- 测试辅助函数 ---

// 注册并登录用户，返回 (token, user_id)
async fn get_auth_token(app: &Router) -> Result<(String, Uuid)> {
    let username = format!("testuser_{}", Uuid::new_v4());
    let email = format!("test_{}@example.com", Uuid::new_v4());
    let password = "password123";

    let register_payload = serde_json::json!({
        "username": username,
        "email": email,
        "password": password
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&register_payload)?))?,
        )
        .await?;

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

    let token = body_json["token"]
        .as_str()
        .context("token not found")?
        .to_string();
    let user_id = Uuid::parse_str(
        body_json["user"]["id"]
            .as_str()
            .context("user.id not found")?,
    )?;

    Ok((token, user_id))
}

// 在数据库中创建一个用户并返回其ID
async fn seed_one_user(pool: &PgPool) -> Result<Uuid> {
    let user_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO users (id, username, email, hashed_password) VALUES ($1, $2, $3, 'fake_hash')"#,
        user_id,
        format!("user_{}", user_id),
        format!("user_{}@example.com", user_id)
    ).execute(pool).await?;
    Ok(user_id)
}

// 更新后的帖子生成函数，必须传入 author_id
async fn seed_one_post(
    pool: &PgPool,
    author_id: Uuid,
    title: &str,
    published: bool,
) -> Result<Post> {
    let slug = slugify(title);
    let published_at: Option<DateTime<Utc>> = if published { Some(Utc::now()) } else { None };
    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (id, slug, title, content, author_id, published_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, slug, title, content, author_id, created_at, updated_at, published_at
        "#,
        Uuid::new_v4(),
        slug,
        title,
        "Some content",
        author_id,
        published_at
    )
    .fetch_one(pool)
    .await
    .context(format!("Seeding post '{}' failed", title))?;
    Ok(post)
}

// 更新后的批量帖子生成函数
async fn seed_posts(pool: &PgPool, author_id: Uuid, count: i32) -> Result<Vec<Post>> {
    let mut posts = Vec::new();
    for i in 0..count {
        let title = format!("Test Post {}", i + 1);
        let post = seed_one_post(pool, author_id, &title, true).await?;
        posts.push(post);
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    }
    posts.sort_by(|a, b| b.created_at.cmp(&a.created_at)); // 按创建时间降序排
    Ok(posts)
}

// 辅助函数，用于创建分类和标签
async fn seed_one_category(pool: &PgPool, name: &str) -> Result<Category> {
    let category = sqlx::query_as!(
        Category,
        r#"INSERT INTO categories (id, name, slug) VALUES ($1, $2, $3) RETURNING *"#,
        Uuid::new_v4(),
        name,
        slugify(name)
    )
    .fetch_one(pool)
    .await?;
    Ok(category)
}

async fn seed_one_tag(pool: &PgPool, name: &str) -> Result<Tag> {
    let tag = sqlx::query_as!(
        Tag,
        r#"INSERT INTO tags (id, name, slug) VALUES ($1, $2, $3) RETURNING *"#,
        Uuid::new_v4(),
        name,
        slugify(name)
    )
    .fetch_one(pool)
    .await?;
    Ok(tag)
}

// --- 更新后的测试用例 ---

// --- 分页测试 ---
#[sqlx::test]
async fn test_list_posts_default_pagination(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let author_id = seed_one_user(&pool).await?; // 创建作者
    let seeded_posts = seed_posts(&pool, author_id, 15).await?; // 传入作者ID

    let request = Request::builder().uri("/posts").body(Body::empty())?;
    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let page_response: PaginatedResponse<PostDetailDto> = serde_json::from_slice(&body_bytes)?;

    assert_eq!(page_response.total_items, 15);
    assert_eq!(page_response.page, 1);
    assert_eq!(page_response.items.len(), 10);
    assert_eq!(page_response.items[0].id, seeded_posts[0].id);

    Ok(())
}

// --- 创建帖子测试 ---
#[sqlx::test]
async fn test_create_post_valid_payload(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let (token, user_id) = get_auth_token(&app).await?; // 获取 token 和 user_id

    let payload = CreatePostPayload {
        title: "新帖子标题".to_string(),
        content: "这是新帖子的内容。".to_string(),
        category_ids: None,
        tag_ids: None,
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/posts")
        .header("content-type", "application/json")
        .header("Authorization", format!("Bearer {}", token)) // 添加认证头
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::CREATED);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let created_post_dto: PostDetailDto = serde_json::from_slice(&body_bytes)?;

    // 从数据库验证 author_id
    let saved_post = sqlx::query_as!(
        Post,
        "SELECT * FROM posts WHERE id = $1",
        created_post_dto.id
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(saved_post.author_id, Some(user_id));
    assert_eq!(saved_post.title, payload.title);

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
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/posts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED); // 验证返回 401

    Ok(())
}

// --- 获取帖子测试 ---
#[sqlx::test]
async fn test_get_post_by_id_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let author_id = seed_one_user(&pool).await?;
    let seeded_post = seed_one_post(&pool, author_id, "获取测试标题ID", true).await?;

    let request = Request::builder()
        .uri(format!("/posts/{}", seeded_post.id))
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched_post: PostDetailDto = serde_json::from_slice(&body_bytes)?;
    assert_eq!(fetched_post.id, seeded_post.id);

    Ok(())
}

// --- 更新帖子测试 ---
#[sqlx::test]
async fn test_update_post_full_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    // 注意: 更新和删除帖子的接口我们还未用代码保护，所以暂时不需要 token
    // 如果未来保护了，这里的测试也需要像 create_post 那样获取和添加 token
    let author_id = seed_one_user(&pool).await?;
    let original_post = seed_one_post(&pool, author_id, "原始标题", false).await?;

    let update_payload = UpdatePostPayload {
        title: Some("更新后的标题".to_string()),
        content: Some("更新后的内容".to_string()),
        ..Default::default()
    };

    let request_uri = format!("/posts/{}", original_post.id);
    let request = Request::builder()
        .method(Method::PUT)
        .uri(request_uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&update_payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let db_post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", original_post.id)
        .fetch_one(&pool)
        .await?;
    assert_eq!(db_post.title, "更新后的标题");
    assert_eq!(db_post.author_id, Some(author_id)); // 作者不应改变

    Ok(())
}

// --- 删除帖子测试 ---
#[sqlx::test]
async fn test_delete_post_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let author_id = seed_one_user(&pool).await?;
    let post_to_delete = seed_one_post(&pool, author_id, "待删除的帖子", true).await?;

    let request = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/posts/{}", post_to_delete.id))
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

// --- 关联关系测试 ---
#[sqlx::test]
async fn test_create_post_with_categories_and_tags(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let (token, _user_id) = get_auth_token(&app).await?;

    let category1 = seed_one_category(&pool, "测试分类1").await?;
    let tag1 = seed_one_tag(&pool, "测试标签1").await?;

    let payload = CreatePostPayload {
        title: "带关联的帖子".to_string(),
        content: "内容...".to_string(),
        category_ids: Some(vec![category1.id]),
        tag_ids: Some(vec![tag1.id]),
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/posts")
        .header("content-type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::CREATED);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let post_detail: PostDetailDto = serde_json::from_slice(&body_bytes)?;

    assert!(post_detail.categories.is_some());
    assert_eq!(post_detail.categories.as_ref().unwrap().len(), 1);
    assert_eq!(post_detail.categories.as_ref().unwrap()[0].id, category1.id);
    assert!(post_detail.tags.is_some());
    assert_eq!(post_detail.tags.as_ref().unwrap().len(), 1);

    Ok(())
}
