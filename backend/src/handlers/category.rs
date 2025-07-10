use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::api_error::ApiError;
use crate::auth::AuthUser;
use crate::dtos::category::{
    BatchDeleteCategoriesPayload, CreateCategoryPayload, MergeCategoriesPayload,
    MergeCategoriesPreviewPayload, UpdateCategoryPayload,
};
use crate::handlers::AppState;
use uuid::Uuid;

/// 创建新分类的 Handler
pub async fn create_category_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 任何认证用户都可以创建分类
    auth_user.require_permission("category:create")?;
    tracing::info!("接收到创建分类请求：{:?}", payload);

    // 调用 CategoryService 的create_category方法
    let category = state.category_service.create_category(payload).await?;
    tracing::info!("分类创建成功：{:?}", category);

    Ok((StatusCode::CREATED, Json(category)))
}

/// 获取所有分类列表的 Handler
pub async fn list_categories_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("接收到获取分类列表请求");
    let categories = state.category_service.list_categories().await?;
    tracing::info!("成功获取 {} 个分类", categories.len());

    Ok(Json(categories))
}

/// 获取单个分类的Handler（可以用过 ID 或 Slug）
pub async fn get_category_handler(
    State(state): State<AppState>,
    Path(id_or_slug): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("接收到获取单个分类请求：{}", id_or_slug);
    let category = match Uuid::try_parse(&id_or_slug) {
        Ok(id) => {
            tracing::info!("参数 '{}' 被解析为 UUID：{}", id_or_slug, id);
            state.category_service.get_category_by_id(id).await?
        }
        Err(_) => {
            tracing::info!("参数 '{}' 被视为 slug", id_or_slug);
            state
                .category_service
                .get_category_by_slug(&id_or_slug)
                .await?
        }
    };
    tracing::info!("成功获取分类：{:?}", category);

    Ok(Json(category))
}

/// 更新分类的 Handler
pub async fn update_category_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查
    auth_user.require_permission("category:manage")?;

    tracing::info!("接收到更新分类请求：ID:{},Payload:{:?}", id, payload);
    let updated_category = state.category_service.update_category(id, payload).await?;
    tracing::info!("分类更新成功: {:?}", updated_category);

    Ok(Json(updated_category))
}

/// 删除分类的 Handler
pub async fn delete_category_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查
    auth_user.require_permission("category:manage")?;

    tracing::info!("接收到删除分类请求： ID:{}", id);
    state.category_service.delete_category(id).await?;
    tracing::info!("分类删除成功： ID:{}", id);

    Ok(StatusCode::NO_CONTENT)
}

// === 管理员专用的分类管理功能 ===

/// 合并分类：将多个分类合并为一个目标分类
pub async fn merge_categories_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<MergeCategoriesPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要分类管理权限
    auth_user.require_permission("category:manage")?;

    tracing::info!("接收到分类合并请求：{:?}", payload);

    let result = state
        .category_service
        .merge_categories(payload.target_category_id, &payload.source_category_ids)
        .await?;

    tracing::info!("分类合并成功：{:?}", result);

    Ok(Json(result))
}

/// 增强版分类合并：提供详细的操作结果和可选的目标分类重命名
pub async fn merge_categories_enhanced_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<MergeCategoriesPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要分类管理权限
    auth_user.require_permission("category:manage")?;

    tracing::info!("接收到增强分类合并请求：{:?}", payload);

    let result = state
        .category_service
        .merge_categories_enhanced(
            payload.target_category_id,
            &payload.source_category_ids,
            payload.new_target_name.as_deref(),
        )
        .await?;

    tracing::info!("增强分类合并成功：{:?}", result);

    Ok(Json(result))
}

/// 获取分类合并预览
pub async fn get_category_merge_preview_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<MergeCategoriesPreviewPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要分类管理权限
    auth_user.require_permission("category:manage")?;

    tracing::info!("接收到分类合并预览请求：{:?}", payload);

    let preview = state
        .category_service
        .get_merge_preview(payload.target_category_id, &payload.source_category_ids)
        .await?;

    tracing::info!("分类合并预览生成成功");

    Ok(Json(preview))
}

/// 批量删除分类
pub async fn batch_delete_categories_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<BatchDeleteCategoriesPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要分类管理权限
    auth_user.require_permission("category:manage")?;

    tracing::info!("接收到批量删除分类请求：{:?}", payload);

    // 检查是否需要处理孤儿文章
    let handle_orphaned = matches!(
        payload.handle_orphaned_posts,
        Some(crate::dtos::category::OrphanedPostsHandling::AddUncategorizedCategory)
    );

    if handle_orphaned {
        // 使用增强版批量删除
        let response = state
            .category_service
            .batch_delete_categories_enhanced(&payload.category_ids, true)
            .await?;

        tracing::info!("增强版批量删除分类成功：{}", response.operation_summary);
        Ok(Json(response))
    } else {
        // 使用简单版批量删除
        let deleted_count = state
            .category_service
            .batch_delete_categories(&payload.category_ids)
            .await?;

        tracing::info!("批量删除分类成功：删除了 {} 个分类", deleted_count);

        // 返回与增强版相同的响应结构
        let response = crate::dtos::category::BatchDeleteCategoriesResponse {
            deleted_count,
            affected_post_count: 0, // 简单版本不统计受影响的文章
            orphaned_post_count: 0,
            operation_summary: format!("成功删除 {} 个分类", deleted_count),
        };

        Ok(Json(response))
    }
}

/// 获取分类使用统计
pub async fn get_category_usage_stats_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要分类管理权限
    auth_user.require_permission("category:manage")?;

    tracing::info!("接收到获取分类使用统计请求");

    let stats = state.category_service.get_category_usage_stats().await?;

    tracing::info!("成功获取 {} 个分类的使用统计", stats.len());

    Ok(Json(stats))
}

/// 查找相似分类
pub async fn find_similar_categories_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要分类管理权限
    auth_user.require_permission("category:manage")?;

    tracing::info!("接收到查找相似分类请求");

    let similar_groups = state.category_service.find_similar_categories().await?;

    tracing::info!("找到 {} 组相似分类", similar_groups.len());

    Ok(Json(similar_groups))
}
