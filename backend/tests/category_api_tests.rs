use anyhow::{Context, Result};
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    Router,
};
use backend::{
    dtos::category::{CreateCategoryPayload, UpdateCategoryPayload}, // DTOs for Category
    handlers::AppState,
    models::Category, // Category model
    repositories::{
        CategoryRepository, PostRepository, // Category repository
        PostgresCategoryRepository, PostgresPostRepository,
        PostgresTagRepository, TagRepository,
    },
    routes::create_router,
    services::{CategoryService, PostService, TagService}, // Category service
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
        let default_filter = "info,backend=trace,sqlx=warn";
        let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|e| {
            eprintln!(
                "[测试日志初始化警告] 解析 RUST_LOG 失败 ('{}'), 将使用默认过滤规则: '{}'",
                e, default_filter
            );
            EnvFilter::new(default_filter)
        });

        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_test_writer()
            .try_init()
            .ok();
    });
}

// --- 辅助函数：设置测试环境 (包含所有服务) ---
// 可以复用 tag_api_tests.rs 中的 setup_test_app_for_tags，或者创建一个通用的
// 为了清晰，这里创建一个相似的，但可以考虑重构为一个共享的 setup 函数
async fn setup_test_app_for_categories(pool: PgPool) -> Router {
    ensure_tracing_is_initialized_for_test();

    let post_repo: Arc<dyn PostRepository> =
        Arc::new(PostgresPostRepository::new(pool.clone()));
    let post_service = Arc::new(PostService::new(post_repo));

    let category_repo: Arc<dyn CategoryRepository> =
        Arc::new(PostgresCategoryRepository::new(pool.clone()));
    let category_service = Arc::new(CategoryService::new(category_repo));

    let tag_repo: Arc<dyn TagRepository> = Arc::new(PostgresTagRepository::new(pool.clone()));
    let tag_service = Arc::new(TagService::new(tag_repo));

    let app_state = AppState {
        post_service,
        category_service,
        tag_service,
    };

    create_router(app_state)
}

// --- 辅助函数：在测试数据库中创建单个分类 ---
async fn seed_one_category(pool: &PgPool, name: &str) -> Result<Category> {
    let slug = slugify(name);
    let category = sqlx::query_as!(
        Category,
        r#"
        INSERT INTO categories (id, name, slug)
        VALUES ($1, $2, $3)
        RETURNING id, name, slug, created_at, updated_at
        "#,
        Uuid::new_v4(),
        name,
        slug
    )
        .fetch_one(pool)
        .await
        .context(format!("Seeding category '{}' failed", name))?;
    Ok(category)
}

// --- 辅助函数：在测试数据库中创建多个分类 ---
async fn seed_categories(pool: &PgPool, names: Vec<&str>) -> Result<Vec<Category>> {
    let mut categories = Vec::new();
    for name in names {
        let category = seed_one_category(pool, name).await?;
        categories.push(category);
    }
    Ok(categories)
}

// --- 测试 POST /categories (创建分类) ---
#[sqlx::test]
async fn test_create_category_valid_payload(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool.clone()).await;
    let payload = CreateCategoryPayload { // 使用 CreateCategoryPayload
        name: Some("测试分类".to_string()), // CreateCategoryPayload 的 name 是 Option<String>
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/categories")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let created_category: Category = serde_json::from_slice(&body_bytes) // 反序列化为 Category
        .context("无法将响应体反序列化为 Category")?;

    assert_eq!(created_category.name, *payload.name.as_ref().unwrap()); // 注意 payload.name 是 Option
    assert!(!created_category.slug.is_empty());
    assert_eq!(created_category.slug, slugify(payload.name.as_ref().unwrap()));

    // 直接从数据库验证
    let db_category = sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = $1", created_category.id)
        .fetch_one(&pool)
        .await?;
    assert_eq!(db_category.name, *payload.name.as_ref().unwrap());

    Ok(())
}

#[sqlx::test]
async fn test_create_category_empty_name_in_option(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool).await;
    let payload = CreateCategoryPayload {
        name: Some("".to_string()), // 空名称，但 Some()
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/categories")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    let status = response.status();
    // CategoryService 中对空名称的 trim().is_empty() 应该捕获这个
    assert!(
        status == StatusCode::INTERNAL_SERVER_ERROR || status == StatusCode::BAD_REQUEST,
        "预期空名称(Some(\"\"))返回错误状态码, 实际为: {}", status
    );
    if status != StatusCode::INTERNAL_SERVER_ERROR {
        let body_bytes = response.into_body().collect().await?.to_bytes();
        let error_response: serde_json::Value = serde_json::from_slice(&body_bytes)?;
        assert!(error_response["error"].as_str().unwrap_or("").contains("分类名称不能为空"));
    }
    Ok(())
}

#[sqlx::test]
async fn test_create_category_none_name(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool).await;
    let payload = CreateCategoryPayload {
        name: None, // 名称字段为 None
    };
    // 根据你的 CreateCategoryPayload 定义，name 是 Option<String>。
    // CategoryService 的 create_category 方法中：
    // let name_owned = payload.name.ok_or_else(|| anyhow!("分类名称不能为空"))?;
    // 这会因为 payload.name 是 None 而直接返回错误。

    let request = Request::builder()
        .method(Method::POST)
        .uri("/categories")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?; // payload 会是 {"name":null}

    let response = app.oneshot(request).await?;
    let status = response.status();

    assert!(
        status == StatusCode::INTERNAL_SERVER_ERROR || status == StatusCode::BAD_REQUEST,
        "预期 name:None 返回错误状态码, 实际为: {}", status
    );
    if status != StatusCode::INTERNAL_SERVER_ERROR {
        let body_bytes = response.into_body().collect().await?.to_bytes();
        let error_response: serde_json::Value = serde_json::from_slice(&body_bytes)?;
        assert!(error_response["error"].as_str().unwrap_or("").contains("分类名称不能为空"));
    }
    Ok(())
}


#[sqlx::test]
async fn test_create_category_name_conflict(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool.clone()).await;
    let category_name = "唯一的分类";

    let _first_category = seed_one_category(&pool, category_name).await?;

    let payload_conflict = CreateCategoryPayload {
        name: Some(category_name.to_string()),
    };

    let request_conflict = Request::builder()
        .method(Method::POST)
        .uri("/categories")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload_conflict)?))?;

    let response_conflict = app.oneshot(request_conflict).await?;

    assert_eq!(
        response_conflict.status(),
        StatusCode::BAD_REQUEST,
        "预期名称冲突返回 BAD_REQUEST"
    );
    Ok(())
}

// --- 测试 GET /categories (获取分类列表) ---
#[sqlx::test]
async fn test_list_categories_empty(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool).await;
    let request = Request::builder().uri("/categories").body(Body::empty())?;
    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let categories: Vec<Category> = serde_json::from_slice(&body_bytes)?;
    assert!(categories.is_empty());
    Ok(())
}

#[sqlx::test]
async fn test_list_categories_multiple(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool.clone()).await;
    let cat_names = vec!["技术", "生活", "随笔"];
    let mut seeded_categories = seed_categories(&pool, cat_names).await?;
    seeded_categories.sort_by_key(|c| c.name.clone()); // API 默认按名称排序

    let request = Request::builder().uri("/categories").body(Body::empty())?;
    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched_categories: Vec<Category> = serde_json::from_slice(&body_bytes)?;

    assert_eq!(fetched_categories.len(), seeded_categories.len());
    for i in 0..seeded_categories.len() {
        assert_eq!(fetched_categories[i].id, seeded_categories[i].id);
        assert_eq!(fetched_categories[i].name, seeded_categories[i].name);
    }
    Ok(())
}

// --- 测试 GET /categories/{identifier} (获取单个分类) ---
#[sqlx::test]
async fn test_get_category_by_id_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool.clone()).await;
    let seeded_category = seed_one_category(&pool, "ID获取测试").await?;

    let request = Request::builder().uri(format!("/categories/{}", seeded_category.id)).body(Body::empty())?;
    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched: Category = serde_json::from_slice(&body_bytes)?;
    assert_eq!(fetched.id, seeded_category.id);
    Ok(())
}

#[sqlx::test]
async fn test_get_category_by_slug_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool.clone()).await;
    let seeded_category = seed_one_category(&pool, "Slug获取测试").await?;

    let request = Request::builder().uri(format!("/categories/{}", seeded_category.slug)).body(Body::empty())?;
    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let fetched: Category = serde_json::from_slice(&body_bytes)?;
    assert_eq!(fetched.slug, seeded_category.slug);
    Ok(())
}

#[sqlx::test]
async fn test_get_category_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool).await;
    let random_uuid = Uuid::new_v4();
    let non_existent_slug = "不存在的分类-slug";

    // Test by ID
    let request_id = Request::builder().uri(format!("/categories/{}", random_uuid)).body(Body::empty())?;
    let response_id = app.clone().oneshot(request_id).await?;
    assert_eq!(response_id.status(), StatusCode::NOT_FOUND);

    // Test by slug
    let request_slug = Request::builder().uri(format!("/categories/{}", non_existent_slug)).body(Body::empty())?;
    let response_slug = app.oneshot(request_slug).await?;
    assert_eq!(response_slug.status(), StatusCode::NOT_FOUND);
    Ok(())
}


// --- 测试 PUT /categories/{id} (更新分类) ---
#[sqlx::test]
async fn test_update_category_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool.clone()).await;
    let original_category = seed_one_category(&pool, "旧分类名").await?;

    let payload = UpdateCategoryPayload { // UpdateCategoryPayload
        name: Some("新分类名".to_string()),
    };
    let expected_new_slug = slugify("新分类名");

    let request = Request::builder()
        .method(Method::PUT)
        .uri(format!("/categories/{}", original_category.id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await?.to_bytes();
    let updated: Category = serde_json::from_slice(&body_bytes)?;

    assert_eq!(updated.id, original_category.id);
    assert_eq!(updated.name, "新分类名");
    assert_eq!(updated.slug, expected_new_slug);
    assert!(updated.updated_at > original_category.updated_at);
    Ok(())
}

#[sqlx::test]
async fn test_update_category_empty_name(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool.clone()).await;
    let original_category = seed_one_category(&pool, "有效分类名").await?;
    let payload = UpdateCategoryPayload { name: Some("".to_string()) };

    let request = Request::builder()
        .method(Method::PUT)
        .uri(format!("/categories/{}", original_category.id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    let status = response.status();
    assert!(
        status == StatusCode::INTERNAL_SERVER_ERROR || status == StatusCode::BAD_REQUEST,
        "预期更新为empty name返回错误, 实际为: {}", status
    );
    if status != StatusCode::INTERNAL_SERVER_ERROR {
        let body_bytes = response.into_body().collect().await?.to_bytes();
        let error_response: serde_json::Value = serde_json::from_slice(&body_bytes)?;
        assert!(error_response["error"].as_str().unwrap_or("").contains("分类名称不能为空"));
    }
    Ok(())
}

#[sqlx::test]
async fn test_update_category_none_name(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool.clone()).await;
    let original_category = seed_one_category(&pool, "有名字的分类").await?;
    // UpdateCategoryPayload 的 name 是 Option<String>
    // CategoryService 的 update_category 如果 payload.name 是 None (或者没有提供 name 字段导致 serde 反序列化为 None)
    // 它的逻辑是：
    // if new_name_opt.is_none() { return self.get_category_by_id(id).await ... }
    // 所以，如果发送 {"name": null} 或 {} (如果UpdateCategoryPayload有 #[serde(default)])
    // 它会尝试获取并返回原始分类（如果没其他字段更新的话）。
    // 这意味着状态码会是 OK，并且返回的是未修改的分类。
    let payload = UpdateCategoryPayload { name: None }; //  {"name": null}
    let empty_payload: UpdateCategoryPayload = Default::default(); // {}

    // Test with name: None
    let request_none = Request::builder()
        .method(Method::PUT)
        .uri(format!("/categories/{}", original_category.id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?; // {"name":null}

    let response_none = app.clone().oneshot(request_none).await?;
    assert_eq!(response_none.status(), StatusCode::OK, "预期name:None时，返回原分类和OK状态");
    let body_bytes_none = response_none.into_body().collect().await?.to_bytes();
    let fetched_cat_none: Category = serde_json::from_slice(&body_bytes_none)?;
    assert_eq!(fetched_cat_none.id, original_category.id);
    assert_eq!(fetched_cat_none.name, original_category.name);


    // Test with empty payload (if UpdateCategoryPayload derives Default)
    // payload.name will be None
    let request_empty = Request::builder()
        .method(Method::PUT)
        .uri(format!("/categories/{}", original_category.id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&empty_payload)?))?; // {}

    let response_empty = app.clone().oneshot(request_empty).await?;
    assert_eq!(response_empty.status(), StatusCode::OK, "预期空payload时，返回原分类和OK状态");
    let body_bytes_empty = response_empty.into_body().collect().await?.to_bytes();
    let fetched_cat_empty: Category = serde_json::from_slice(&body_bytes_empty)?;
    assert_eq!(fetched_cat_empty.id, original_category.id);
    assert_eq!(fetched_cat_empty.name, original_category.name);

    Ok(())
}


#[sqlx::test]
async fn test_update_category_name_conflict(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool.clone()).await;
    let _cat1 = seed_one_category(&pool, "已存在分类").await?;
    let cat_to_update = seed_one_category(&pool, "要改名的分类").await?;
    let payload = UpdateCategoryPayload { name: Some("已存在分类".to_string()) };

    let request = Request::builder()
        .method(Method::PUT)
        .uri(format!("/categories/{}", cat_to_update.id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    Ok(())
}

#[sqlx::test]
async fn test_update_category_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool).await;
    let non_existent_uuid = Uuid::new_v4();
    let payload = UpdateCategoryPayload { name: Some("随意名称".to_string()) };

    let request = Request::builder()
        .method(Method::PUT)
        .uri(format!("/categories/{}", non_existent_uuid))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload)?))?;

    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

// --- 测试 DELETE /categories/{id} (删除分类) ---
#[sqlx::test]
async fn test_delete_category_success(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool.clone()).await;
    let cat_to_delete = seed_one_category(&pool, "要删除的分类").await?;

    let request = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/categories/{}", cat_to_delete.id))
        .body(Body::empty())?;

    let response = app.clone().oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let maybe_deleted = sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = $1", cat_to_delete.id)
        .fetch_optional(&pool).await?;
    assert!(maybe_deleted.is_none());
    Ok(())
}

#[sqlx::test]
async fn test_delete_category_not_found(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool).await;
    let non_existent_uuid = Uuid::new_v4();

    let request = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/categories/{}", non_existent_uuid))
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[sqlx::test]
async fn test_delete_category_invalid_uuid_format(pool: PgPool) -> Result<()> {
    let app = setup_test_app_for_categories(pool).await;
    let invalid_uuid = "这不是uuid";
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/categories/{}", invalid_uuid))
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST, "预期删除无效UUID格式返回 400");
    Ok(())
}