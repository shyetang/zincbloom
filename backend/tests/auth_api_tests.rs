use anyhow::Result;
use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode},
};
use backend::services::auth::LoginTokens;
use backend::services::{
    AdminService, AuthService, CategoryService, EmailService, PostService, TagService, UserService,
};
use backend::{
    config::{AppConfig, AuthConfig, DatabaseConfig, DraftPolicy, EmailConfig, ServerConfig},
    dtos::auth::{LoginResponsePayload, RefreshTokenPayload},
    handlers::AppState,
    routes::create_router,
};
use http_body_util::BodyExt;
use sqlx::PgPool;
use std::sync::{Arc, Once};
use tower::ServiceExt;
use tracing_subscriber::EnvFilter;

// --- 测试环境设置  ---
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
            jwt_secret: "test_secret".to_string(),
            jwt_issuer: "test_issuer".to_string(),
            jwt_audience: "test_audience".to_string(),
            access_token_expiry_minutes: 15,
            refresh_token_expiry_days: 7,
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

    let user_repo = Arc::new(backend::repositories::PostgresUserRepository::new(
        pool.clone(),
    ));
    let role_repo = Arc::new(backend::repositories::PostgresRoleRepository::new(
        pool.clone(),
    ));
    let permission_repo = Arc::new(backend::repositories::PostgresPermissionRepository::new(
        pool.clone(),
    ));
    let one_time_token_repo = Arc::new(backend::repositories::PostgresOneTimeTokenRepository::new(
        pool.clone(),
    ));
    let login_attempt_repo = Arc::new(backend::repositories::PostgresLoginAttemptRepository::new(
        pool.clone(),
    ));

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
    let user_service = Arc::new(UserService::new(user_repo));
    let post_service = Arc::new(PostService::new(
        Arc::new(backend::repositories::PostgresPostRepository::new(
            pool.clone(),
        )),
        Arc::new(backend::repositories::PostgresCategoryRepository::new(
            pool.clone(),
        )),
        Arc::new(backend::repositories::PostgresTagRepository::new(
            pool.clone(),
        )),
    ));

    let app_state = AppState {
        post_service,
        category_service: Arc::new(CategoryService::new(Arc::new(
            backend::repositories::PostgresCategoryRepository::new(pool.clone()),
        ))),
        tag_service: Arc::new(TagService::new(Arc::new(
            backend::repositories::PostgresTagRepository::new(pool.clone()),
        ))),
        auth_service,
        admin_service,
        user_service,
    };

    create_router(app_state)
}

// --- 认证流程的核心测试用例 ---

#[sqlx::test]
async fn test_register_login_flow(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;
    let username = "testuser_reg_login";
    let email = "reg_login@example.com";
    let password = "StrongPassword123!";

    // 1. 测试注册
    let register_payload =
        serde_json::json!({ "username": username, "email": email, "password": password });
    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_payload)?))?;

    let response = app.clone().oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::CREATED);

    // 2. 测试登录
    let login_payload = serde_json::json!({ "username": username, "password": password });
    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&login_payload)?))?;

    let response = app.clone().oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    // 3. 验证登录响应
    let body_bytes = response.into_body().collect().await?.to_bytes();
    let login_response: LoginResponsePayload = serde_json::from_slice(&body_bytes)?;

    assert!(!login_response.access_token.is_empty());
    assert!(!login_response.refresh_token.is_empty());
    assert_eq!(login_response.user.username, username);

    Ok(())
}

#[sqlx::test]
async fn test_password_policy_failures(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;
    let base_payload = |password: &str| {
        serde_json::json!({
            "username": "testuser_policy",
            "email": "policy@example.com",
            "password": password
        })
    };

    // 测试密码太短
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&base_payload("Short1!"))?))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // 测试缺少大写字母
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&base_payload(
                    "longpassword1!",
                ))?))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // ... 可以为其他策略添加更多测试 ...

    Ok(())
}

#[sqlx::test]
async fn test_refresh_token_flow(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;

    // 1. 先登录获取一套token
    let login_payload =
        serde_json::json!({ "username": "testuser", "password": "StrongPassword123!" });
    // (假设用户已通过其他方式注册，或在setup中预置)
    app.clone().oneshot(Request::builder().method(Method::POST).uri("/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&serde_json::json!({
            "username": "testuser", "email": "refresh@example.com", "password": "StrongPassword123!"
        }))?))?).await?;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&login_payload)?))?,
        )
        .await?;

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let initial_tokens: LoginResponsePayload = serde_json::from_slice(&body_bytes)?;
    let old_refresh_token = initial_tokens.refresh_token;

    // 2. 使用 refresh token 去获取新的 token
    let refresh_payload = RefreshTokenPayload {
        refresh_token: old_refresh_token.clone(),
    };
    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/refresh")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&refresh_payload)?))?;

    let response = app.clone().oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    // 3. 验证返回了新的一套 token
    let body_bytes = response.into_body().collect().await?.to_bytes();
    let new_tokens: LoginTokens = serde_json::from_slice(&body_bytes)?;

    assert_ne!(
        new_tokens.access_token, initial_tokens.access_token,
        "Access token 应该被刷新"
    );
    assert_ne!(
        new_tokens.refresh_token, old_refresh_token,
        "Refresh token 应该被轮换"
    );

    // 4. (重要) 验证旧的 refresh token 是否已失效
    let refresh_payload_old = RefreshTokenPayload {
        refresh_token: old_refresh_token,
    };
    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/refresh")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&refresh_payload_old)?))?;

    let response = app.clone().oneshot(request).await?;
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "旧的 refresh token 应该已失效"
    );

    Ok(())
}

#[sqlx::test]
async fn test_logout_flow(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;

    // 1. 登录获取 refresh_token
    // (省略注册和登录代码，复用上面的逻辑)
    let login_payload =
        serde_json::json!({ "username": "testuser_logout", "password": "StrongPassword123!" });
    app.clone().oneshot(Request::builder().method(Method::POST).uri("/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&serde_json::json!({
            "username": "testuser_logout", "email": "logout@example.com", "password": "StrongPassword123!"
        }))?))?).await?;
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&login_payload)?))?,
        )
        .await?;
    let body_bytes = response.into_body().collect().await?.to_bytes();
    let tokens: LoginResponsePayload = serde_json::from_slice(&body_bytes)?;
    let refresh_token_to_logout = tokens.refresh_token;

    // 2. 调用注销接口
    let logout_payload = RefreshTokenPayload {
        refresh_token: refresh_token_to_logout.clone(),
    };
    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/logout")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&logout_payload)?))?;

    let response = app.clone().oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 3. 验证刚刚被注销的 refresh token 是否已失效
    let refresh_payload_logged_out = RefreshTokenPayload {
        refresh_token: refresh_token_to_logout,
    };
    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/refresh")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(
            &refresh_payload_logged_out,
        )?))?;

    let response = app.clone().oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}
