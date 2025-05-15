use anyhow::{Context, Result};
use backend::config::AppConfig;
use backend::handlers::AppState;
use backend::repositories::{CategoryRepository, PostRepository, PostgresCategoryRepository, PostgresPostRepository};
use backend::routes::create_router;
use backend::services::{CategoryService, PostService};
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
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = AppConfig::from_env().context("加载应用配置失败")?;

    // 设置数据库连接池
    let db_pool = PgPool::connect(&config.database.url)
        .await
        .context("连接数据库失败")?;
    tracing::info!("数据库连接池已连接。");

    // (可选) 运行数据库迁移
    /*sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .context("运行数据库迁移失败")?;
    tracing::info!("数据库迁移已应用。");*/

    //  -- 依赖注入 --
    //  -- 创建 PostService 实例 ---
    let post_repo = Arc::new(PostgresPostRepository::new(db_pool.clone()));
    let post_repo_trait: Arc<dyn PostRepository> = post_repo;
    let post_service = Arc::new(PostService::new(post_repo_trait));
    
    //  -- 创建 CategoryService 实例 ---
    let category_repo = Arc::new(PostgresCategoryRepository::new(db_pool.clone()));
    let category_repo_trait: Arc<dyn CategoryRepository> = category_repo;
    let category_service = Arc::new(CategoryService::new(category_repo_trait));
    
    // 创建 AppState
    let app_state = AppState { post_service, category_service };

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
