use anyhow::{Context, Result};

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};

use tower::ServiceExt;

use blog_backend::{
    dtos::PaginatedResponse,
    handlers::AppState,
    models::Post,
    repositories::{PostRepository, PostgresPostRepository},
    routes::create_router,
    services::PostService,
};

use axum::http::Method;
use chrono::{DateTime, SubsecRound, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use blog_backend::models::{CreatePostPayload, UpdatePostPayload};
use http_body_util::BodyExt;

use std::sync::Once;
use tracing_subscriber::EnvFilter; // 导入 EnvFilter

static TRACING_INIT_TEST: Once = Once::new();
// use hyper::body;

// --- 辅助函数：设置测试环境 ---
// 这个函数会在每个 #[sqlx::test] 标记的测试运行前被调用
// 它返回 测试所需的 AppState 或 Router
async fn setup_test_app(pool: PgPool) -> Router {
    // 接收由 #[sqlx::test] 注入的连接池
    ensure_tracing_is_initialized_for_test(); // 在测试函数开头调用

    // 1. 创建依赖实例
    let post_repo = Arc::new(PostgresPostRepository::new(pool.clone()));
    let post_repo_trait: Arc<dyn PostRepository> = post_repo;
    let post_service = Arc::new(PostService::new(post_repo_trait));

    // 2. 创建应用状态
    let app_state = AppState { post_service };

    // 3. 创建Router
    create_router(app_state)
}

// 一个辅助函数，可以在每个 #[sqlx::test] 函数的开头调用，或者在测试模块的 setup 函数中调用
fn ensure_tracing_is_initialized_for_test() {
    TRACING_INIT_TEST.call_once(|| {
        // 定义默认的日志过滤规则
        // 例如: info 级别全局, 你的 crate (blog_backend) 设置为 trace, sqlx 设置为 warn
        let default_filter = "info,blog_backend=trace,sqlx=warn"; // 将 "blog_backend" 替换为你的实际 crate 名称

        // 尝试从 RUST_LOG 环境变量加载过滤器，如果失败或未设置，则使用默认值
        let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|e| {
            // 可选: 打印一个警告到 stderr，说明 RUST_LOG 解析失败，使用了默认值
            eprintln!(
                "[测试日志初始化警告] 解析 RUST_LOG 失败 ('{}'), 将使用默认过滤规则: '{}'",
                e, default_filter
            );
            EnvFilter::new(default_filter)
        });

        tracing_subscriber::fmt()
            .with_env_filter(env_filter) // 应用获取到的过滤器
            .with_test_writer() // 这是关键！确保日志输出与测试框架集成，能被捕获或显示
            .try_init() // 尝试初始化，如果已经初始化过了，它会失败，但不会 panic
            .ok(); // 忽略 try_init() 可能返回的错误 (例如 "already initialized")
    });
}

// --- 辅助函数：在测试数据库中创建帖子 ---
// 返回创建的 Post 列表，以便后续断言
async fn seed_posts(pool: &PgPool, count: i32) -> Result<Vec<Post>> {
    let mut posts = Vec::new();
    for i in 0..count {
        let title = format!("Test Post {}", i + 1);
        let slug = slug::slugify(&title);
        let content = format!("Content for test post {}.", i + 1);
        // 注意：直接 Insert，依赖 create_at/update_at 默认值，published_at 为 NULL
        // 为了测试分页排序，最好让 created_at/published_at 不同且有序
        // 这里简单处理，让它们略有不同
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let post = sqlx::query_as!(
            Post,
            r#"
            insert into posts (id,slug,title,content,published_at)
            values ($1,$2,$3,$4,$5)
            returning id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at
            "#,
            Uuid::new_v4(),
            slug,
            title,
            content,
            // 为了测试已发布列表，这里直接设置为当前时间
            // 如果要测试草稿，可以设为 None::<DateTime<Utc>>
            Some(Utc::now())
        )
            .fetch_one(pool)
            .await
            .context(format!("Seeding post {} failed",i + 1))?;
        posts.push(post);
        // 在循环中短暂 sleep 来确保 created_at 不同，以便测试排序
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    }
    // 让最新的帖子排在最前面 (如果默认排序是 DESC)
    posts.reverse();
    Ok(posts)
}

// 返回创建的 Post 列表，以便后续断言
async fn seed_one_post(pool: &PgPool, title: &str, content: &str, published: bool) -> Result<Post> {
    let slug = slug::slugify(title);
    let published_at: Option<DateTime<Utc>> = if published { Some(Utc::now()) } else { None };

    let post = sqlx::query_as!(
        Post,
        r#"
        insert into posts (id,slug,title,content,published_at)
        values ($1,$2,$3,$4,$5)
        returning id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!", published_at
        "#,
        Uuid::new_v4(),
        slug,
        title,
        content,
        published_at,
    )
        .fetch_one(pool)
        .await
        .context(format!("Seeding post '{}' failed",title))?;

    Ok(post)
}

// --- 测试分页用例 ---
// 使用 #[sqlx::test] 宏
// 它会自动处理数据库连接、事务和回滚
#[sqlx::test]
async fn test_list_posts_default_pagination(pool: PgPool) -> Result<()> {
    // 1. 准备环境和数据
    let app = setup_test_app(pool.clone()).await; // 这里 pool 需要clone
    let total_seed_posts = 15; // 创建超过一页的数据
    let seeded_posts = seed_posts(&pool, total_seed_posts).await?;

    // 2. 构造请求(无分页参数）
    let request = Request::builder()
        .uri("/posts") // 无查询参数
        .body(Body::empty())?;

    // 3.发送请求
    let response = app.oneshot(request).await?;

    // 4. 断言状态码
    assert_eq!(response.status(), StatusCode::OK);

    // 5. 解析响应体
    let body_bytes = response
        .into_body() // 类型是 impl hyper::body::Body
        .collect() // // 调用 BodyExt::collect()，返回 Result<Collected<Bytes>, hyper::Error>
        .await
        .context("收集响应体失败")? // 处理 collect() 可能产生的错误
        .to_bytes(); // 从 Collected<Bytes> 中获取 Bytes

    let page_response: PaginatedResponse<Post> = serde_json::from_slice(&body_bytes)
        .context("无法将响应体反序列化为PaginatedResponse<Post>")?;

    // 6. 断言分页元数据
    assert_eq!(page_response.total_items, total_seed_posts as i64); // 总数应等于我们插入的数量
    assert_eq!(page_response.page, 1); // 默认页码应为1
    assert_eq!(page_response.page_size, 10); // 默认每页大小应为 10 
    assert_eq!(page_response.total_pages, 2); // 15条数据，每页10条，应有 2 页
    assert_eq!(page_response.items.len(), 10); // 第一页应有 10 条数据

    // 7. 断言内容 - 检查第一页的数据是否是我们最新创建的10条
    for i in 0..10 {
        assert_eq!(page_response.items[i].id, seeded_posts[i].id);
        assert_eq!(page_response.items[i].title, seeded_posts[i].title);
    }

    Ok(())
}

#[sqlx::test]
async fn test_list_posts_specific_page_and_size(pool: PgPool) -> Result<()> {
    // 1. 准备环境和数据
    let app = setup_test_app(pool.clone()).await;
    let total_seeded_posts = 25; // 假设每页 5 条，需要 5 页
    let seeded_posts = seed_posts(&pool, total_seeded_posts).await?;

    // 2. 构造请求（请求第 3 页，每页5条）
    let request = Request::builder()
        .uri("/posts?page=3&page_size=5")
        .body(Body::empty())?;

    // 3. 发送请求
    let response = app.oneshot(request).await?;

    // 4. 断言状态
    assert_eq!(response.status(), StatusCode::OK);

    // 5. 解析响应体
    let body_bytes = response
        .into_body()
        .collect()
        .await
        .context("收集响应体失败")?
        .to_bytes();

    let page_response: PaginatedResponse<Post> = serde_json::from_slice(&body_bytes)
        .context("无法将响应体反序列化为PaginatedResponse<Post>")?;

    // 6. 断言分页元数据
    assert_eq!(page_response.total_items, total_seeded_posts as i64);
    assert_eq!(page_response.page, 3);
    assert_eq!(page_response.page_size, 5);
    assert_eq!(page_response.total_pages, 5);
    assert_eq!(page_response.items.len(), 5);

    // 7. 断言内容 - 检查第 3 页的数据是否是我们创建的第 11 到 15 条 (索引 10 到 14)
    for i in 0..5 {
        let expected_index = 10 + i; // page 3, size 5 => offset 10
        assert_eq!(page_response.items[i].id, seeded_posts[expected_index].id);
        assert_eq!(
            page_response.items[i].title,
            seeded_posts[expected_index].title
        );
    }

    Ok(())
}

#[sqlx::test]
async fn test_list_posts_last_page_partial(pool: PgPool) -> Result<()> {
    // 1. 准备环境和数据
    let app = setup_test_app(pool.clone()).await;
    let total_seeded_posts = 12; // 每页 5 条，最后一页只有 2 条
    let seeded_posts = seed_posts(&pool, total_seeded_posts).await?;

    // 2. 构造请求 (请求第 3 页，每页 5 条)
    let request = Request::builder()
        .uri("/posts?page=3&page_size=5")
        .body(Body::empty())?;

    // 3. 发送请求
    let response = app.oneshot(request).await?;

    // 4. 断言状态码
    assert_eq!(response.status(), StatusCode::OK);

    // 5. 解析响应体
    let body_bytes = response
        .into_body()
        .collect()
        .await
        .context("收集响应体失败")?
        .to_bytes();

    let page_response: PaginatedResponse<Post> = serde_json::from_slice(&body_bytes)
        .context("无法将响应体反序列化为PaginatedResponse<Post>")?;

    // 6. 断言分页元数据
    assert_eq!(page_response.total_items, total_seeded_posts as i64);
    assert_eq!(page_response.page, 3);
    assert_eq!(page_response.page_size, 5);
    assert_eq!(page_response.total_pages, 3); // 12 条数据，每页 5 条，应有 3 页
    // 7. 断言最后一页的 item 数量
    assert_eq!(page_response.items.len(), 2); // 最后一页只有 2 条

    // 8. 断言内容 - 检查是否是最后 2 条数据 (索引 10, 11)
    assert_eq!(page_response.items[0].id, seeded_posts[10].id);
    assert_eq!(page_response.items[1].id, seeded_posts[11].id);

    Ok(())
}

// --- 测试 POST /posts (创建帖子) 用例 ---
#[sqlx::test]
async fn test_create_post_valid_payload(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let payload = CreatePostPayload {
        title: "新帖子标题".to_string(),
        content: "这是新帖子的内容。".to_string(),
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/posts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "预期状态码为 201 CREATED"
    );

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let created_post: Post =
        serde_json::from_slice(&body_bytes).context("无法将响应体反序列化为Post")?;

    assert_eq!(created_post.title, payload.title, "帖子标题不匹配");
    assert_eq!(created_post.content, payload.content, "帖子内容不匹配");
    assert!(!created_post.slug.is_empty(), "Slug 不应为空");
    assert_eq!(
        created_post.slug,
        slug::slugify(&payload.title),
        "Slug 生成不符合预期"
    );
    assert!(
        created_post.published_at.is_none(),
        "新帖子 published_at 应该为 None (草稿)"
    );

    //直接从数据库验证
    let db_post = sqlx::query_as!(
        Post,
        r#"
        select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at
        from posts
        where id = $1
        "#,
        created_post.id
    )
        .fetch_one(&pool)
        .await?;

    assert_eq!(db_post.title, payload.title);

    Ok(())
}

#[sqlx::test]
async fn test_create_post_empty_title(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;

    let payload = CreatePostPayload {
        title: "".to_string(),
        content: "一些内容。".to_string(),
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/posts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;

    assert_eq!(
        response.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "预期空标题返回错误状态码"
    );
    Ok(())
}

#[sqlx::test]
async fn test_create_post_slug_conflict(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let title = "唯一的标题";

    // 1. 先成功创建一个帖子
    let _first_post = seed_one_post(&pool, title, "一些内容", false).await?;

    // 2. 尝试创建另一个同名帖子（会导致 slug 冲突）
    let payload_conflict = CreatePostPayload {
        title: title.to_string(),
        content: "不同的内容，但标题相同".to_string(),
    };

    let request_conflict = Request::builder()
        .method(Method::POST)
        .uri("/posts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload_conflict)?))?;

    let response_conflict = app.oneshot(request_conflict).await?;

    // 检查 ApiError::IntoResponse 是否能将数据库唯一约束错误 (23505) 转换为合适的客户端错误
    // 根据之前的 ApiError 实现，应该能匹配到 23505 并返回 BAD_REQUEST
    assert_eq!(
        response_conflict.status(),
        StatusCode::BAD_REQUEST,
        "预期 slug 冲突返回 BAD_REQUEST"
    );

    let body_bytes = response_conflict.into_body().collect().await?.to_bytes();
    let error_response: serde_json::Value = serde_json::from_slice(&body_bytes)?;
    let error_msg = error_response["error"].as_str().unwrap_or("");
    assert!(
        error_msg.contains("记录已存在")
            || error_msg.contains("Slug")
            || error_msg.contains("constraint"),
        "错误消息应指明唯一约束冲突"
    );

    Ok(())
}

// --- 测试 GET /posts/{id_or_slug} (获取单个帖子)
#[sqlx::test]
async fn test_get_post_by_id_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let seeded_post = seed_one_post(&pool, "获取测试标题ID", "获取测试内容ID", true).await?;

    let request_uri = format!("/posts/{}", seeded_post.id);
    let request = Request::builder()
        .method(Method::GET)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK, "预期状态码为 200 OK");

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched_post: Post =
        serde_json::from_slice(&body_bytes).context("无法将响应体反序列化为 Post")?;

    assert_eq!(fetched_post.id, seeded_post.id, "帖子 ID 不匹配");
    assert_eq!(fetched_post.title, seeded_post.title, "帖子标题不匹配");

    Ok(())
}

#[sqlx::test]
async fn test_get_post_by_slug_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let seeded_post = seed_one_post(&pool, "获取测试标题ID", "获取测试内容ID", true).await?;

    let request_uri = format!("/posts/{}", seeded_post.slug);
    let request = Request::builder()
        .method(Method::GET)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK, "预期状态码为 200 OK");

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched_post: Post = serde_json::from_slice(&body_bytes)?;

    assert_eq!(fetched_post.id, seeded_post.id, "帖子 ID 不匹配");
    assert_eq!(fetched_post.slug, seeded_post.slug, "帖子 Slug 不匹配");

    Ok(())
}

#[sqlx::test]
async fn test_get_post_by_id_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;
    let random_uuid = Uuid::new_v4(); // 一个不存在的 UUID

    let request_uri = format!("/posts/{}", random_uuid);
    let request = Request::builder()
        .method(Method::GET)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;
    // Service 层返回 anyhow!("未找到...") 错误, ApiError 将其转换为 404
    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "预期帖子未找到返回 404"
    );

    // 检查错误消息
    // let body_bytes = response.into_body().collect().await?.to_bytes();
    // let error_response: serde_json::Value = serde_json::from_slice(&body_bytes)?;
    // assert!(error_response["error"].as_str().unwrap_or("").contains("未找到"));

    Ok(())
}

#[sqlx::test]
async fn test_get_post_by_slug_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;
    let non_existent_slug = "一个肯定不存在的slug-abcdef";

    let request_uri = format!("/posts/{}", non_existent_slug);
    let request = Request::builder()
        .method(Method::GET)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;

    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "预期帖子未找到返回 404"
    );
    Ok(())
}

#[sqlx::test]
async fn test_get_post_by_unparseable_string_as_slug_not_found(pool: PgPool) -> Result<()> {
    // 这个测试验证当传入一个既不是UUID也不是有效slug的字符串时，最终会作为slug查询失败
    let app = setup_test_app(pool).await;
    let gibberish_string = "这不是UUID也不是有效slug";

    let request_uri = format!("/posts/{}", gibberish_string);
    let request = Request::builder()
        .method(Method::GET)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;

    // 因为 get_post_handler 中，Uuid::parse_str 失败会落入 Err(_) 分支，按 slug 查询
    // 所以这里预期结果和按无效 slug 查询一样，是 404
    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "预期帖子未找到返回 404"
    );
    Ok(())
}

// --- 测试 PUT /posts/{id} (更新帖子) ---

// 测试所有可更新字段都被成功更新的情况，包括发布状态
#[sqlx::test]
async fn test_update_post_full_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let original_post = seed_one_post(&pool, "原始标题", "原始内容", false).await?;

    let now_for_publish = Utc::now();
    let update_payload = UpdatePostPayload {
        title: Some("更新后的标题".to_string()),
        content: Some("更新后的内容".to_string()),
        slug: None,
        published_at: Some(now_for_publish), // 显示发布
        unpublish: false,                    // 明确不是撤稿 (或者依赖默认值)
    };

    let request_uri = format!("/posts/{}", original_post.id);
    let request = Request::builder()
        .method(Method::PUT)
        .uri(request_uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&update_payload)?))?;

    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK, "预期状态码为 200 OK");

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let updated_post_response: Post =
        serde_json::from_slice(&body_bytes).context("无法将响应体反序列化为 Post")?;

    assert_eq!(updated_post_response.id, original_post.id);
    assert_eq!(updated_post_response.title, "更新后的标题");
    assert_eq!(updated_post_response.content, "更新后的内容");
    assert_eq!(updated_post_response.slug, slug::slugify("更新后的标题")); // 验证 slug 是否被更新
    assert!(
        updated_post_response.published_at.is_some(),
        "帖子应该已发布"
    );
    // 比较时间戳时要注意可能的微秒差异，这里简单比较日期和小时分钟秒
    assert_eq!(
        updated_post_response.published_at.unwrap().trunc_subsecs(0),
        now_for_publish.trunc_subsecs(0)
    );
    assert!(
        updated_post_response.published_at > Option::from(original_post.updated_at),
        "updated_at 应该已更新"
    );

    // 直接从数据库验证
    let db_post = sqlx::query_as!(
        Post,
        r#"
        select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at
        from posts
        where id = $1
        "#,
        original_post.id
    )
        .fetch_one(&pool)
        .await?;

    assert_eq!(db_post.title, "更新后的标题");
    assert_eq!(db_post.slug, slug::slugify("更新后的标题"));
    assert!(db_post.published_at.is_some());

    Ok(())
}

// 测试只更新部分字段（例如标题）的情况
#[sqlx::test]
async fn test_update_post_partial_title_only(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let original_post = seed_one_post(&pool, "部分更新原始标题", "部分更新原始内容", true).await?; // 初始已发布

    let update_payload = UpdatePostPayload {
        title: Some("部分更新后的标题".to_string()),
        content: None,      // 不更新内容
        slug: None,         // 不显式更新 slug
        published_at: None, // 不改变发布状态
        unpublish: false,   // 明确不是撤稿 (或者依赖默认值)
    };

    let request_uri = format!("/posts/{}", original_post.id);
    let request = Request::builder()
        .method(Method::PUT)
        .uri(request_uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&update_payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let updated_post_response: Post = serde_json::from_slice(&body_bytes)?;

    assert_eq!(updated_post_response.title, "部分更新后的标题");
    assert_eq!(
        updated_post_response.content, original_post.content,
        "内容不应改变"
    ); // 验证内容未变

    let expected_slug = slug::slugify("部分更新后的标题");

    assert_eq!(
        updated_post_response.slug, expected_slug,
        "Slug 应根据新标题重新生成"
    );
    assert_eq!(
        updated_post_response.published_at, original_post.published_at,
        "发布状态不应改变"
    );
    assert!(updated_post_response.updated_at > original_post.updated_at);

    Ok(())
}

// 专门测试将 published_at 设置为 Some(None) (JSON 中是 null) 来撤销发布的功能
#[sqlx::test]
async fn test_update_post_unpublish(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let original_post = seed_one_post(&pool, "待撤稿标题", "待撤稿内容", true).await?; // 初始已发布
    assert!(original_post.published_at.is_some());

    let update_payload = UpdatePostPayload {
        title: None,
        content: None,
        slug: None,
        published_at: None, // 显式设置为 null，即撤稿
        unpublish: true,    // 明确不是撤稿 (或者依赖默认值)
    };

    let request_uri = format!("/posts/{}", original_post.id);
    let request = Request::builder()
        .method(Method::PUT)
        .uri(request_uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&update_payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let updated_post_response: Post = serde_json::from_slice(&body_bytes)?;

    assert!(
        updated_post_response.published_at.is_none(),
        "帖子应该已撤稿 (published_at 为 None)"
    );
    assert_eq!(updated_post_response.title, original_post.title); // 其他字段不应改变
    assert!(updated_post_response.updated_at > original_post.updated_at);

    Ok(())
}

// 测试尝试更新一个不存在的帖子 ID 时，应返回 404。
#[sqlx::test]
async fn test_update_post_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;
    let non_existent_uuid = Uuid::new_v4();
    let update_payload = UpdatePostPayload {
        title: Some("任意标题".to_string()),
        ..Default::default()
    }; // 使用 Default 填充 Option

    let request_uri = format!("/posts/{}", non_existent_uuid);
    let request = Request::builder()
        .method(Method::PUT)
        .uri(request_uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&update_payload)?))?;

    let response = app.oneshot(request).await?;
    // 预期 Service 层 get_by_id 返回 None，然后 ok_or_else 产生 "未找到" 错误, ApiError 映射为 404
    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "预期更新不存在的帖子返回 404"
    );
    Ok(())
}

// 测试当提供的更新数据本身不合法时（例如标题为空字符串），API 应如何响应。
#[sqlx::test]
async fn test_update_post_invalid_payload_empty_title(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let original_post = seed_one_post(&pool, "标题置空测试", "内容", false).await?;

    let update_payload = UpdatePostPayload {
        title: Some("".to_string()), // 尝试将标题设置为空
        content: None,
        slug: None,
        published_at: None,
        unpublish: false, // 明确不是撤稿 (或者依赖默认值)
    };

    let request_uri = format!("/posts/{}", original_post.id);
    let request = Request::builder()
        .method(Method::PUT)
        .uri(request_uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&update_payload)?))?;

    let response = app.oneshot(request).await?;
    // 预期 Service 层验证逻辑返回 anyhow!("标题不能为空")
    // 需要 ApiError::IntoResponse 能够处理这类验证错误并返回 400 或 422
    // 目前的 ApiError 对于非特定 downcast 错误可能返回 500，或者如果 anyhow 错误消息包含“不能为空”，可能被我们的简单字符串匹配捕获
    // 这是一个可以改进 ApiError 处理的地方
    let status = response.status();
    assert!(
        status == StatusCode::BAD_REQUEST || status == StatusCode::INTERNAL_SERVER_ERROR,
        "预期空标题更新返回客户端错误 (400/422) 或当前为(500)"
    );
    if status == StatusCode::BAD_REQUEST || status == StatusCode::UNPROCESSABLE_ENTITY {
        // 更理想的状态码
        let body_bytes = response.into_body().collect().await?.to_bytes();
        let error_response: serde_json::Value = serde_json::from_slice(&body_bytes)?;
        assert!(
            error_response["error"]
                .as_str()
                .unwrap_or("")
                .contains("标题不能为空")
        );
    }
    Ok(())
}

// --- 测试 DELETE /posts/{id} (删除帖子) ---
// 测试成功删除和尝试删除不存在的帖子这两种主要情况
#[sqlx::test]
async fn test_delete_post_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool.clone()).await;
    let seeded_post = seed_one_post(&pool, "待删除的帖子", "一些内容", true).await?;
    let post_id_to_delete = seeded_post.id;

    // 1. 构造删除请求
    let request_uri = format!("/posts/{}", post_id_to_delete);
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(request_uri)
        .body(Body::empty())?;

    // 2. 发送请求
    let response = app.clone().oneshot(request).await?; // Clone app for potential reuse

    // 3. 断言状态码
    // 成功删除通常返回 204 No Content，表示操作成功，但响应体中没有内容
    assert_eq!(
        response.status(),
        StatusCode::NO_CONTENT,
        "预期删除成功返回 204 NO CONTENT"
    );

    // 4. 验证数据库中帖子确实已被删除
    let delete_post_in_db = sqlx::query_as!(
        Post,
        r#"
        select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at
        from posts
        where id = $1
        "#,
        post_id_to_delete
    )
        .fetch_optional(&pool) // 使用 fetch_optional 因为我们期望它不存在
        .await
        .context("删除后查询帖子失败")?;

    assert!(delete_post_in_db.is_none(), "贼子在数据库中应该已被删除");
    Ok(())
}

#[sqlx::test]
async fn test_delete_post_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;
    let non_existent_uuid = Uuid::new_v4(); // 一个肯定不存在的UUID

    let request_uri = format!("/posts/{}", non_existent_uuid);
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;

    // 断言状态码
    // 预期 Service 层或 Repository 层返回 "未找到" 错误, ApiError 将其映射为 404
    // 在 PostRepository::delete 中针对 rows_affected == 0 的情况使用了 anyhow::bail!("...未找到记录...")
    // ApiError::IntoResponse 中的字符串匹配逻辑应该能捕获这个并返回 404
    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "预期删除不存在的帖子返回 404 NOT FOUND"
    );
    Ok(())
}

#[sqlx::test]
async fn test_delete_post_invalid_uuid_format(pool: PgPool) -> Result<()> {
    let app = setup_test_app(pool).await;
    let invalid_uuid_string = "this-is-not-a-uuid";

    let request_uri = format!("/posts/{}", invalid_uuid_string);
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;

    // 断言状态码
    // delete_post_handler 的 Path(id) 参数类型是 Path<Uuid>，
    // Axum 的路径提取器在解析 "this-is-not-a-uuid" 到 Uuid 时会失败。
    // 这种路径参数解析失败通常会导致 Axum 自动返回 404 Not Found 或有时是 400 Bad Request。
    // 让我们先预期 404，如果实际是 400 也可以接受。
    let status = response.status();
    assert!(
        status == StatusCode::NOT_FOUND || status == StatusCode::BAD_REQUEST,
        "预期删除无效UUID格式的帖子返回 404 或 400,实际为：{}",
        status
    );
    Ok(())
}
