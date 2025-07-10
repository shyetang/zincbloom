use crate::api_error::ApiError;
use crate::auth::AuthUser;
use crate::dtos::tag::{
    BatchDeleteTagsPayload, CreateTagPayload, MergeTagsPayload, MergeTagsPreviewPayload,
    UpdateTagPayload,
};
use crate::handlers::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

/// 创建新标签的 Handler
pub async fn create_tag_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,         // 从应用状态中提取服务实例
    Json(payload): Json<CreateTagPayload>, // 从请求体中解析 JSON 数据到 CreateTagPayload
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 任何认证用户都可以创建标签
    auth_user.require_permission("tag:create")?;

    // 返回值可以是任何实现了 IntoResponse 的类型，或者 ApiError
    tracing::info!("接收到创建标签请求：{:?}", payload);

    // 调用 TagService 的 create_tag 方法
    let tag = state.tag_service.create_tag(payload).await?;

    tracing::info!("标签创建成功：{:?}", tag);

    // 成功时返回 201 Created 状态码和创建的标签 (JSON格式)
    Ok((StatusCode::CREATED, Json(tag)))
}

/// 获取所有标签列表的 Handler
pub async fn list_tags_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("接收到获取标签列表的请求");

    let tags = state.tag_service.list_tags().await?;

    tracing::info!("成功获取 {} 个标签", tags.len());
    // 成功时返回 200 OK 状态码和标签列表 (JSON格式)
    Ok(Json(tags))
}

/// 获取单个标签的 Handler (可以通过 ID 或 Slug)
pub async fn get_tag_handler(
    State(state): State<AppState>,
    Path(id_or_slug): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("接收到获取单个标签请求：{}", id_or_slug);
    let tag = match Uuid::try_parse(&id_or_slug) {
        Ok(id) => {
            tracing::info!("参数 '{}' 被解析为 UUID：{}", id_or_slug, id);
            state.tag_service.get_tag_by_id(id).await?
        }
        Err(_) => {
            tracing::info!("参数 '{}' 被视为 slug", id_or_slug);
            state.tag_service.get_tag_by_slug(&id_or_slug).await?
        }
    };
    tracing::info!("成功获取标签：{:?}", tag);

    Ok(Json(tag))
}

/// 更新标签的 Handler
pub async fn update_tag_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTagPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查
    auth_user.require_permission("tag:manage")?;

    tracing::info!("接收到更新标签请求：ID：{},Payload: {:?}", id, payload);
    let updated_tag = state.tag_service.update_tag(id, payload).await?;
    tracing::info!("标签更新成功：{:?}", updated_tag);

    Ok(Json(updated_tag))
}

pub async fn delete_tag_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查
    auth_user.require_permission("tag:manage")?;

    tracing::info!("接收到删除标签请求：ID: {}", id);
    state.tag_service.delete_tag(id).await?;
    tracing::info!("标签删除成功：ID: {}", id);

    // 成功删除通常返回 204 No Content 状态码，表示操作成功但响应体中没有内容
    Ok(StatusCode::NO_CONTENT)
}

// === 管理员专用的标签管理功能 ===

/// 合并标签：将多个标签合并为一个目标标签
pub async fn merge_tags_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<MergeTagsPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要标签管理权限
    auth_user.require_permission("tag:manage")?;

    tracing::info!("接收到标签合并请求：{:?}", payload);

    let result = state
        .tag_service
        .merge_tags(payload.target_tag_id, &payload.source_tag_ids)
        .await?;

    tracing::info!("标签合并成功：{:?}", result);

    Ok(Json(result))
}

/// 增强版标签合并：提供详细的操作结果和可选的目标标签重命名
pub async fn merge_tags_enhanced_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<MergeTagsPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要标签管理权限
    auth_user.require_permission("tag:manage")?;

    tracing::info!("接收到增强标签合并请求：{:?}", payload);

    let result = state
        .tag_service
        .merge_tags_enhanced(
            payload.target_tag_id,
            &payload.source_tag_ids,
            payload.new_target_name.as_deref(),
        )
        .await?;

    tracing::info!("增强标签合并成功：{:?}", result);

    Ok(Json(result))
}

/// 获取标签合并预览
pub async fn get_merge_preview_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<MergeTagsPreviewPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要标签管理权限
    auth_user.require_permission("tag:manage")?;

    tracing::info!("接收到标签合并预览请求：{:?}", payload);

    let preview = state
        .tag_service
        .get_merge_preview(payload.target_tag_id, &payload.source_tag_ids)
        .await?;

    tracing::info!("标签合并预览生成成功");

    Ok(Json(preview))
}

/// 批量删除标签
pub async fn batch_delete_tags_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<BatchDeleteTagsPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要标签管理权限
    auth_user.require_permission("tag:manage")?;

    tracing::info!("接收到批量删除标签请求：{:?}", payload);

    let deleted_count = state
        .tag_service
        .batch_delete_tags(&payload.tag_ids)
        .await?;

    tracing::info!("批量删除标签成功：删除了 {} 个标签", deleted_count);

    Ok(Json(serde_json::json!({ "deleted_count": deleted_count })))
}

/// 获取标签使用统计
pub async fn get_tag_usage_stats_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要标签管理权限
    auth_user.require_permission("tag:manage")?;

    tracing::info!("接收到获取标签使用统计请求");

    let stats = state.tag_service.get_tag_usage_stats().await?;

    tracing::info!("成功获取 {} 个标签的使用统计", stats.len());

    Ok(Json(stats))
}

/// 查找相似标签
pub async fn find_similar_tags_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查 - 需要标签管理权限
    auth_user.require_permission("tag:manage")?;

    tracing::info!("接收到查找相似标签请求");

    let similar_groups = state.tag_service.find_similar_tags().await?;

    tracing::info!("找到 {} 组相似标签", similar_groups.len());

    Ok(Json(similar_groups))
}
