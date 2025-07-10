use anyhow::{Context, Result};
use backend::config::AppConfig;
use backend::handlers::AppState;
use backend::repositories::{
    CategoryRepository, LoginAttemptRepository, OneTimeTokenRepository, PermissionRepository,
    PostRepository, PostgresCategoryRepository, PostgresLoginAttemptRepository,
    PostgresOneTimeTokenRepository, PostgresPermissionRepository, PostgresPostRepository,
    PostgresRoleRepository, PostgresTagRepository, PostgresUserRepository, RoleRepository,
    TagRepository, UserRepository,
};
use backend::routes::create_router;
use backend::services::{
    AdminService, AuthService, CategoryService, EmailService, PostService, TagService, UserService,
};
use sqlx::PgPool;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<()> {
    // ---- 初始化日志 ----
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "into".into()))
        .with(tracing_subscriber::fmt::layer().with_timer(
            tracing_subscriber::fmt::time::OffsetTime::local_rfc_3339().expect("获取本地时区失败"),
        ))
        .init();

    // 加载配置
    let config = AppConfig::from_env().context("加载应用配置失败")?;

    tracing::info!("应用程序正在使用的邮件配置: {:?}", config.email);

    // 设置数据库连接池
    let db_pool = PgPool::connect(&config.database.url)
        .await
        .context("连接数据库失败")?;
    tracing::info!("数据库连接池已连接。");

    // (可选) 运行数据库迁移
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .context("运行数据库迁移失败")?;
    tracing::info!("数据库迁移已应用。");

    //  -- 依赖注入 --
    //  -- 实例化所有的Repository ---
    let category_repo: Arc<dyn CategoryRepository> =
        Arc::new(PostgresCategoryRepository::new(db_pool.clone()));
    let tag_repo: Arc<dyn TagRepository> = Arc::new(PostgresTagRepository::new(db_pool.clone()));
    let post_repo: Arc<dyn PostRepository> = Arc::new(PostgresPostRepository::new(db_pool.clone()));
    let user_repo: Arc<dyn UserRepository> = Arc::new(PostgresUserRepository::new(db_pool.clone()));
    let role_repo: Arc<dyn RoleRepository> = Arc::new(PostgresRoleRepository::new(db_pool.clone()));
    let permission_repo: Arc<dyn PermissionRepository> =
        Arc::new(PostgresPermissionRepository::new(db_pool.clone()));
    let login_attempt_repo: Arc<dyn LoginAttemptRepository> =
        Arc::new(PostgresLoginAttemptRepository::new(db_pool.clone()));
    let one_time_token_repo: Arc<dyn OneTimeTokenRepository> =
        Arc::new(PostgresOneTimeTokenRepository::new(db_pool.clone()));

    // -- 实例化所有的 Services ----
    let tag_service = Arc::new(TagService::new(tag_repo.clone()));
    let category_service = Arc::new(CategoryService::new(category_repo.clone()));
    let email_service = Arc::new(EmailService::new(config.email.clone()));

    let auth_service = Arc::new(AuthService::new(
        user_repo.clone(),
        role_repo.clone(),
        login_attempt_repo.clone(),
        one_time_token_repo.clone(),
        email_service.clone(),
        &config,
    ));
    let admin_service = Arc::new(AdminService::new(
        user_repo.clone(),
        role_repo.clone(),
        permission_repo.clone(),
    ));
    let user_service = Arc::new(UserService::new(user_repo.clone()));

    //  -- 创建 PostService 实例 ---
    let post_service = Arc::new(PostService::new(
        post_repo.clone(),
        category_repo.clone(),
        tag_repo.clone(),
        user_repo.clone(),
    ));

    // 创建 AppState
    let app_state = AppState {
        post_service,
        category_service,
        tag_service,
        auth_service,
        admin_service,
        user_service,
    };

    // 创建 Axum 路由
    let app = create_router(app_state);

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("服务器监听于 {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context(format!("绑定端口 {} 失败", addr))?;

    axum::serve(listener, app.into_make_service())
        .await
        .context("启动Axum 服务器失败")?;

    Ok(())
}
