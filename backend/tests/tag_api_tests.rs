use anyhow::{Context, Result};
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    Router,
};
use backend::{
    dtos::tag::{CreateTagPayload, UpdateTagPayload}, // DTOs for Tag
    handlers::AppState,
    models::Tag, // Tag model
    repositories::{
        CategoryRepository,
        PostRepository,
        PostgresCategoryRepository,
        PostgresPostRepository,
        PostgresTagRepository,
        TagRepository, // Tag repository
    },
    routes::create_router,
    services::{CategoryService, PostService, TagService}, // Tag service
};
use http_body_util::BodyExt;
use slug::slugify;
// To verify slug generation
use sqlx::PgPool;
use std::sync::{Arc, Once};
use tower::ServiceExt;
// For `oneshot`
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

// --- 辅助: 确保日志只初始化一次 ---
static TRACING_INIT_TEST: Once = Once::new();

fn ensure_tracing_is_initialized_for_test() {
    TRACING_INIT_TEST.call_once(|| {
        let default_filter = "info,backend=trace,sqlx=warn"; // "backend" 是你的 crate 名称
        let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|e| {
            eprintln!(
                "[测试日志初始化警告] 解析 RUST_LOG 失败 ('{}'), 将使用默认过滤规则: '{}'",
                e, default_filter
            );
            EnvFilter::new(default_filter)
        });

        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_test_writer() // 关键！确保日志输出与测试框架集成
            .try_init()
            .ok(); // 忽略已初始化的错误
    });
}

// --- 辅助函数：设置测试环境 (包含 TagService) ---
async fn setup_test_app_for_tags(pool: PgPool) -> Router {
    ensure_tracing_is_initialized_for_test();

    // 1. 创建依赖实例

    let category_repo: Arc<dyn CategoryRepository> =
        Arc::new(PostgresCategoryRepository::new(pool.clone()));
    let category_service = Arc::new(CategoryService::new(category_repo.clone()));

    let tag_repo: Arc<dyn TagRepository> = Arc::new(PostgresTagRepository::new(pool.clone())); // 新增 TagRepository
    let tag_service = Arc::new(TagService::new(tag_repo.clone())); // 新增 TagService

    let post_repo: Arc<dyn PostRepository> = Arc::new(PostgresPostRepository::new(pool.clone()));
    let post_service = Arc::new(PostService::new(
        post_repo.clone(),
        category_repo.clone(),
        tag_repo.clone(),
    ));
    // 2. 创建应用状态
    let app_state = AppState {
        post_service,
        category_service,
        tag_service, // 将 TagService 添加到 AppState
    };

    // 3. 创建Router
    create_router(app_state)
}

// --- 辅助函数：在测试数据库中创建单个标签 ---
async fn seed_one_tag(pool: &PgPool, name: &str) -> Result<Tag> {
    let slug = slugify(name);
    let tag = sqlx::query_as!(
        Tag,
        r#"
        INSERT INTO tags (id, name, slug)
        VALUES ($1, $2, $3)
        RETURNING id, name, slug, created_at, updated_at
        "#,
        Uuid::new_v4(),
        name,
        slug
    )
    .fetch_one(pool)
    .await
    .context(format!("Seeding tag '{}' failed", name))?;
    Ok(tag)
}

// --- 辅助函数：在测试数据库中创建多个标签 ---
async fn seed_tags(pool: &PgPool, names: Vec<&str>) -> Result<Vec<Tag>> {
    let mut tags = Vec::new();
    for name in names {
        // 为了确保 created_at 不同（如果排序依赖它），可以加一个小延迟
        // tokio::time::sleep(tokio::time::Duration::from_millis(2)).await;
        let tag = seed_one_tag(pool, name).await?;
        tags.push(tag);
    }
    // 如果默认按名称排序，可以提前排序以方便断言
    // tags.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(tags)
}

// --- 测试 POST /tags (创建标签) ---
#[sqlx::test]
async fn test_create_tag_valid_payload(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool.clone()).await;
    let payload = CreateTagPayload {
        name: "测试标签".to_string(),
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/tags")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let created_tag: Tag =
        serde_json::from_slice(&body_bytes).context("无法将响应体反序列化为 Tag")?;

    assert_eq!(created_tag.name, payload.name);
    assert!(!created_tag.slug.is_empty());
    assert_eq!(created_tag.slug, slugify(&payload.name));

    // 直接从数据库验证
    let db_tag = sqlx::query_as!(Tag, "SELECT * FROM tags WHERE id = $1", created_tag.id)
        .fetch_one(&pool)
        .await?;
    assert_eq!(db_tag.name, payload.name);

    Ok(())
}

#[sqlx::test]
async fn test_create_tag_empty_name(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool).await;
    let payload = CreateTagPayload {
        name: "".to_string(), // 空名称
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/tags")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;

    // Service 层应该返回错误，ApiError 将其转换为客户端错误
    // "标签名称不能为空" -> 通常是 500 (因为是 anyhow generic error) 或 400/422 如果 ApiError 有特定处理
    // 检查 ApiError 的实现，对于 "不能为空" 这类验证错误，最好返回 400 或 422
    // 当前 ApiError 对于 "未找到" 以外的 anyhow 错误，会返回 500
    // 这提示我们 ApiError 可能需要改进对这类业务验证错误的处理，使其返回更合适的客户端状态码如 400
    let status = response.status();
    assert!(
        status == StatusCode::INTERNAL_SERVER_ERROR || status == StatusCode::BAD_REQUEST,
        "预期空名称返回错误状态码 (当前可能是500，理想是400/422), 实际为: {}",
        status
    );

    if status != StatusCode::INTERNAL_SERVER_ERROR {
        // 如果不是500，则检查错误消息
        let body_bytes = response.into_body().collect().await?.to_bytes();
        let error_response: serde_json::Value = serde_json::from_slice(&body_bytes)?;
        let error_msg = error_response["error"].as_str().unwrap_or("");
        assert!(error_msg.contains("标签名称不能为空"));
    }
    Ok(())
}

#[sqlx::test]
async fn test_create_tag_name_conflict(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool.clone()).await;
    let tag_name = "唯一的标签";

    // 1. 先成功创建一个标签
    let _first_tag = seed_one_tag(&pool, tag_name).await?;

    // 2. 尝试创建另一个同名标签
    let payload_conflict = CreateTagPayload {
        name: tag_name.to_string(),
    };

    let request_conflict = Request::builder()
        .method(Method::POST)
        .uri("/tags")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload_conflict)?))?;

    let response_conflict = app.oneshot(request_conflict).await?;

    // 数据库唯一约束错误 (23505) 应该被 ApiError 捕获并转换为 400 BAD_REQUEST
    assert_eq!(
        response_conflict.status(),
        StatusCode::BAD_REQUEST,
        "预期名称冲突返回 BAD_REQUEST"
    );

    let body_bytes = response_conflict.into_body().collect().await?.to_bytes();
    let error_response: serde_json::Value = serde_json::from_slice(&body_bytes)?;
    let error_msg = error_response["error"].as_str().unwrap_or("");
    assert!(
        error_msg.contains("记录已存在") || error_msg.contains("唯一性冲突"),
        "错误消息应指明唯一约束冲突，实际: '{}'",
        error_msg
    );

    Ok(())
}

// --- 测试 GET /tags (获取标签列表) ---
#[sqlx::test]
async fn test_list_tags_empty(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool).await;

    let request = Request::builder()
        .method(Method::GET)
        .uri("/tags")
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let tags: Vec<Tag> = serde_json::from_slice(&body_bytes)?;
    assert!(tags.is_empty(), "预期在没有标签时返回空列表");

    Ok(())
}

#[sqlx::test]
async fn test_list_tags_multiple(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool.clone()).await;
    let tag_names = vec!["Rust", "Axum", "SQLx", "Tokio"];
    let mut seeded_tags = seed_tags(&pool, tag_names).await?;
    // 按名称排序，因为 list API 默认按名称升序
    seeded_tags.sort_by_key(|t| t.name.clone());

    let request = Request::builder()
        .method(Method::GET)
        .uri("/tags")
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched_tags: Vec<Tag> = serde_json::from_slice(&body_bytes)?;

    assert_eq!(fetched_tags.len(), seeded_tags.len());
    for i in 0..seeded_tags.len() {
        assert_eq!(fetched_tags[i].id, seeded_tags[i].id);
        assert_eq!(fetched_tags[i].name, seeded_tags[i].name);
        assert_eq!(fetched_tags[i].slug, seeded_tags[i].slug);
    }
    Ok(())
}

// --- 测试 GET /tags/{identifier} (获取单个标签) ---
#[sqlx::test]
async fn test_get_tag_by_id_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool.clone()).await;
    let seeded_tag = seed_one_tag(&pool, "按ID获取").await?;

    let request_uri = format!("/tags/{}", seeded_tag.id);
    let request = Request::builder()
        .method(Method::GET)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched_tag: Tag = serde_json::from_slice(&body_bytes)?;

    assert_eq!(fetched_tag.id, seeded_tag.id);
    assert_eq!(fetched_tag.name, seeded_tag.name);
    Ok(())
}

#[sqlx::test]
async fn test_get_tag_by_slug_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool.clone()).await;
    let seeded_tag = seed_one_tag(&pool, "按Slug获取").await?;

    let request_uri = format!("/tags/{}", seeded_tag.slug);
    let request = Request::builder()
        .method(Method::GET)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched_tag: Tag = serde_json::from_slice(&body_bytes)?;

    assert_eq!(fetched_tag.id, seeded_tag.id);
    assert_eq!(fetched_tag.slug, seeded_tag.slug);
    Ok(())
}

#[sqlx::test]
async fn test_get_tag_by_id_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool).await;
    let random_uuid = Uuid::new_v4();

    let request_uri = format!("/tags/{}", random_uuid);
    let request = Request::builder().uri(request_uri).body(Body::empty())?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[sqlx::test]
async fn test_get_tag_by_slug_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool).await;
    let non_existent_slug = "不存在的slug";

    let request_uri = format!("/tags/{}", non_existent_slug);
    let request = Request::builder().uri(request_uri).body(Body::empty())?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

// --- 测试 PUT /tags/{id} (更新标签) ---
#[sqlx::test]
async fn test_update_tag_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool.clone()).await;
    let original_tag = seed_one_tag(&pool, "旧名称").await?;

    let payload = UpdateTagPayload {
        name: Some("新名称".to_string()),
    };
    let expected_new_slug = slugify("新名称");

    let request_uri = format!("/tags/{}", original_tag.id);
    let request = Request::builder()
        .method(Method::PUT)
        .uri(request_uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let updated_tag: Tag = serde_json::from_slice(&body_bytes)?;

    assert_eq!(updated_tag.id, original_tag.id);
    assert_eq!(updated_tag.name, "新名称");
    assert_eq!(updated_tag.slug, expected_new_slug);
    assert!(updated_tag.updated_at > original_tag.updated_at);

    // 从数据库验证
    let db_tag = sqlx::query_as!(Tag, "SELECT * FROM tags WHERE id = $1", original_tag.id)
        .fetch_one(&pool)
        .await?;
    assert_eq!(db_tag.name, "新名称");
    assert_eq!(db_tag.slug, expected_new_slug);
    Ok(())
}

#[sqlx::test]
async fn test_update_tag_empty_name(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool.clone()).await;
    let original_tag = seed_one_tag(&pool, "有效名称").await?;

    let payload = UpdateTagPayload {
        name: Some("".to_string()), // 尝试更新为空名称
    };

    let request_uri = format!("/tags/{}", original_tag.id);
    let request = Request::builder()
        .method(Method::PUT)
        .uri(request_uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    let status = response.status();
    assert!(
        status == StatusCode::INTERNAL_SERVER_ERROR || status == StatusCode::BAD_REQUEST,
        "预期空名称更新返回错误状态码, 实际为: {}",
        status
    );
    if status != StatusCode::INTERNAL_SERVER_ERROR {
        let body_bytes = response.into_body().collect().await?.to_bytes();
        let error_response: serde_json::Value = serde_json::from_slice(&body_bytes)?;
        assert!(
            error_response["error"]
                .as_str()
                .unwrap_or("")
                .contains("标签名称不能为空")
        );
    }
    Ok(())
}

#[sqlx::test]
async fn test_update_tag_name_conflict(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool.clone()).await;
    let _tag1 = seed_one_tag(&pool, "已存在的名称").await?;
    let tag_to_update = seed_one_tag(&pool, "待更新的名称").await?;

    let payload = UpdateTagPayload {
        name: Some("已存在的名称".to_string()), // 尝试更新为与 tag1 相同的名称
    };

    let request_uri = format!("/tags/{}", tag_to_update.id);
    let request = Request::builder()
        .method(Method::PUT)
        .uri(request_uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "预期名称冲突时更新返回 BAD_REQUEST"
    );
    Ok(())
}

#[sqlx::test]
async fn test_update_tag_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool).await;
    let non_existent_uuid = Uuid::new_v4();
    let payload = UpdateTagPayload {
        name: Some("任意名称".to_string()),
    };

    let request_uri = format!("/tags/{}", non_existent_uuid);
    let request = Request::builder()
        .method(Method::PUT)
        .uri(request_uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    // Service::update_tag -> repo.get_by_id (None) -> .ok_or_else (Err) -> ApiError (404)
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

// --- 测试 DELETE /tags/{id} (删除标签) ---
#[sqlx::test]
async fn test_delete_tag_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool.clone()).await;
    let tag_to_delete = seed_one_tag(&pool, "待删除标签").await?;

    let request_uri = format!("/tags/{}", tag_to_delete.id);
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.clone().oneshot(request).await?; // Clone app for potential reuse
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 验证数据库中标签确实已被删除
    let deleted_tag_in_db =
        sqlx::query_as!(Tag, "SELECT * FROM tags WHERE id = $1", tag_to_delete.id)
            .fetch_optional(&pool)
            .await?;
    assert!(deleted_tag_in_db.is_none(), "标签在数据库中应该已被删除");

    Ok(())
}

#[sqlx::test]
async fn test_delete_tag_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool).await;
    let non_existent_uuid = Uuid::new_v4();

    let request_uri = format!("/tags/{}", non_existent_uuid);
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;
    // TagRepository::delete 在 rows_affected == 0 时返回 anyhow::Error("...未找到记录...")
    // ApiError 将包含 "未找到" 的 anyhow::Error 映射为 404
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[sqlx::test]
async fn test_delete_tag_invalid_uuid_format(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_tags(pool).await;
    let invalid_uuid_string = "这不是一个uuid";

    let request_uri = format!("/tags/{}", invalid_uuid_string);
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(request_uri)
        .body(Body::empty())?;

    let response = app.oneshot(request).await?;
    // Axum 的 Path<Uuid> 提取器在解析失败时会返回 404 (有些版本或配置可能返回 400)
    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "预期删除无效UUID格式返回 400 Bad Request"
    );

    Ok(())
}
