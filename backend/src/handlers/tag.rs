use crate::api_error::ApiError;
use crate::dtos::{CreateTagPayload, UpdateTagPayload};
use crate::handlers::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use crate::auth::AuthUser;

/// 创建新标签的 Handler
pub async fn create_tag_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,         // 从应用状态中提取服务实例
    Json(payload): Json<CreateTagPayload>, // 从请求体中解析 JSON 数据到 CreateTagPayload
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查
    auth_user.require_permission("category:manage")?;
    
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
    auth_user.require_permission("category:manage")?;
    
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
    auth_user.require_permission("category:manage")?;
    
    tracing::info!("接收到删除标签请求：ID: {}", id);
    state.tag_service.delete_tag(id).await?;
    tracing::info!("标签删除成功：ID: {}", id);

    // 成功删除通常返回 204 No Content 状态码，表示操作成功但响应体中没有内容
    Ok(StatusCode::NO_CONTENT)
}
