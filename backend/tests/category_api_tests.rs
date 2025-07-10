use anyhow::{Context, Result};
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    Router,
};
use backend::{
    config::{AppConfig, AuthConfig, DatabaseConfig, DraftPolicy, EmailConfig, ServerConfig},
    dtos::category::{CreateCategoryPayload, UpdateCategoryPayload},
    handlers::AppState,
    models::{Category, Role, User},
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
    utils::hash_password,
};
use http_body_util::BodyExt;
use slug::slugify;
use sqlx::PgPool;
use std::sync::{Arc, Once};
use tower::ServiceExt;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

// --- 日志和应用设置 (与 post_api_tests.rs 一致) ---
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

async fn setup_test_app(pool: PgPool) -> Router {
    ensure_tracing_is_initialized_for_test();
    let test_config = AppConfig {
        database: DatabaseConfig { url: String::new() },
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        auth: AuthConfig {
            jwt_secret: "test_secret_for_categories".to_string(),
            jwt_issuer: "test_issuer".to_string(),
            jwt_audience: "test_audience".to_string(),
            access_token_expiry_minutes: 5,
            refresh_token_expiry_days: 1,
            max_login_failures: 5,
            lockout_duration_seconds: 900,
        },
        email: EmailConfig {
            smtp_host: "localhost".to_string(),
            smtp_port: 1025,
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
    ));
    let app_state = AppState {
        post_service,
        category_service,
        tag_service,
        auth_service,
        admin_service,
        user_service,
    };
    create_router(app_state)
}

// --- 认证和数据Seeding辅助函数 (与 post_api_tests.rs 一致) ---

/// 注册一个新用户，并赋予指定角色
async fn seed_user_with_role(pool: &PgPool, name: &str, role_name: &str) -> Result<User> {
    let role = sqlx::query_as!(Role, "SELECT * FROM roles WHERE name = $1", role_name)
        .fetch_optional(pool)
        .await?
        .context(format!("Role '{}' not found", role_name))?;
    let hashed_password = hash_password("StrongPassword123!")?;
    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (id, username, email, hashed_password) VALUES ($1, $2, $3, $4) RETURNING *"#,
        Uuid::new_v4(), 
        name, 
        format!("{}@example.com", name), hashed_password
    ).fetch_one(pool).await?;
    sqlx::query!(
        "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)",
        user.id,
        role.id
    )
        .execute(pool)
        .await?;
    Ok(user)
}

/// 为指定用户登录并获取token
async fn get_token_for_user(app: &Router, username: &str, password: &str) -> Result<String> {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(
                    &serde_json::json!({ "username": username, "password": password }),
                )?))?,
        )
        .await?;
    let body_bytes = response.into_body().collect().await?.to_bytes();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes)?;
    Ok(body_json["access_token"]
        .as_str()
        .context("access_token not found")?
        .to_string())
}

/// 注册一个随机的普通用户并登录
async fn register_and_login_new_user(app: &Router) -> Result<(String, Uuid)> {
    let username = format!("user_{}", Uuid::new_v4());
    let email = format!("{}@example.com", &username);
    let password = "StrongPassword123!";

    let register_payload = serde_json::json!({
        "username": &username,
        "email": &email,
        "password": &password
    });

    let register_response = app.clone().oneshot(
        Request::builder()
            .method(Method::POST)
            .uri("/auth/register")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&register_payload)?))?,
    ).await?;

    if register_response.status() != StatusCode::CREATED {
        return Err(anyhow::anyhow!("Failed to register user for test"));
    }

    let login_response = app.clone().oneshot(
        Request::builder()
            .method(Method::POST)
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&serde_json::json!({
                "username": &username, "password": &password
            }))?))?,
    ).await?;

    let body_bytes = login_response.into_body().collect().await?.to_bytes();
    let login_json: serde_json::Value = serde_json::from_slice(&body_bytes)?;

    let token = login_json["access_token"].as_str().context("token not in login response")?.to_string();
    let user_id = Uuid::parse_str(login_json["user"]["id"].as_str().context("user.id not in login response")?)?;

    Ok((token, user_id))
}

/// 在数据库中创建单个分类
async fn seed_one_category(pool: &PgPool, name: &str) -> Result<Category> {
    sqlx::query_as!(
        Category,
        r#"INSERT INTO categories (id, name, slug) VALUES ($1, $2, $3) RETURNING *"#,
        Uuid::new_v4(),
        name,
        slugify(name)
    )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

// --- 分类API集成测试 ---

// == 公开读取操作 (GET)
#[sqlx::test]
async fn test_list_categories_success(pool: PgPool) -> Result<()> {
    // 准备
    let app = setup_test_app(pool.clone()).await;
    seed_one_category(&pool, "技术").await?;
    seed_one_category(&pool, "生活").await?;

    // 执行
    let request = Request::builder().uri("/categories").body(Body::empty())?;
    let response = app.oneshot(request).await?;

    // 断言
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = response.into_body().collect().await?.to_bytes();
    let categories: Vec<Category> = serde_json::from_slice(&body_bytes)?;
    assert_eq!(categories.len(), 2);

    Ok(())
}

#[sqlx::test]
async fn test_get_category_by_slug_success(pool: PgPool) -> Result<()> {
    // 准备
    let app = setup_test_app(pool.clone()).await;
    let seeded_category = seed_one_category(&pool, "一个独特的分类").await?;

    // 执行
    let request = Request::builder()
        .uri(format!("/categories/{}", seeded_category.slug))
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;

    // 断言
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched: Category = serde_json::from_slice(&body_bytes)?;
    assert_eq!(fetched.id, seeded_category.id);

    Ok(())
}

// == 写入/修改操作 (需要 'editor' 或 'admin' 权限)

#[sqlx::test]
async fn test_create_category_as_editor_success(pool: PgPool) -> Result<()> {
    // 准备: 创建一个 "editor" 用户并获取 token
    let app = setup_test_app(pool.clone()).await;
    let editor = seed_user_with_role(&pool, "editor_for_cat", "editor").await?;
    let token = get_token_for_user(&app, &editor.username, "StrongPassword123!").await?;
    let payload = CreateCategoryPayload {
        name: Some("由编辑创建".to_string()),
    };

    // 执行
    let request = Request::builder()
        .method(Method::POST)
        .uri("/categories")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_vec(&payload)?))?;
    let response = app.oneshot(request).await?;

    // 断言
    assert_eq!(response.status(), StatusCode::CREATED);
    Ok(())
}

#[sqlx::test]
async fn test_update_category_as_editor_success(pool: PgPool) -> Result<()> {
    // 准备
    let app = setup_test_app(pool.clone()).await;
    let editor = seed_user_with_role(&pool, "editor_for_cat_update", "editor").await?;
    let token = get_token_for_user(&app, &editor.username, "StrongPassword123!").await?;
    let category = seed_one_category(&pool, "旧分类").await?;
    let payload = UpdateCategoryPayload {
        name: Some("新分类".to_string()),
    };

    // 执行
    let request = Request::builder()
        .method(Method::PUT)
        .uri(format!("/categories/{}", category.id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_vec(&payload)?))?;
    let response = app.oneshot(request).await?;

    // 断言
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = response.into_body().collect().await?.to_bytes();
    let updated: Category = serde_json::from_slice(&body_bytes)?;
    assert_eq!(updated.name, "新分类");
    Ok(())
}

#[sqlx::test]
async fn test_delete_category_as_editor_success(pool: PgPool) -> Result<()> {
    // 准备
    let app = setup_test_app(pool.clone()).await;
    let editor = seed_user_with_role(&pool, "editor_for_cat_delete", "editor").await?;
    let token = get_token_for_user(&app, &editor.username, "StrongPassword123!").await?;
    let category = seed_one_category(&pool, "待删除分类").await?;

    // 执行
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/categories/{}", category.id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;

    // 断言
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}

// == 授权失败测试
#[sqlx::test]
async fn test_create_category_no_token_fails(pool: PgPool) -> Result<()> {
    // 准备
    let app = setup_test_app(pool.clone()).await;
    let payload = CreateCategoryPayload {
        name: Some("无Token分类".to_string()),
    };

    // 执行
    let request = Request::builder()
        .method(Method::POST)
        .uri("/categories")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;
    let response = app.oneshot(request).await?;

    // 断言
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

// 注意：在新的权限系统中，所有注册用户都是 author 角色，
// 而 author 角色有 category:create 权限，所以普通用户可以创建分类。
// 原来的 test_create_category_as_regular_user_fails 测试不再适用，已删除。
