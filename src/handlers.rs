use crate::api_error::ApiError;
use crate::models::{CreatePostPayload, UpdatePostPayload};
use crate::services::PostService;
use anyhow::Result;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use uuid::Uuid;

// 定义应用状态，包含服务实例
// 使用 Arc 来安全地在多个线程间共享服务实例
#[derive(Clone)]
pub struct AppState {
    pub post_service: Arc<PostService>,
}

// Handler 返回 Result<impl IntoResponse, ApiError>
// ApiError 实现了 From<anyhow::Error>,所以可以在 service 调用后用 '?'
// 创建文章处理器
pub async fn create_post_handler(
    State(state): State<AppState>,  // 从状态中提取PostService
    Json(payload): Json<CreatePostPayload>, // 从请求体解析 JSON
) -> Result<impl IntoResponse, ApiError> { // 返回 Result<impl IntoResponse, ApiError>
    let post = state.post_service.create_post(payload).await?; //调用服务层方法
    Ok((StatusCode::CREATED, Json(post)))   // 成功返回 201 CREATED 和 JSON 数据
}

// 获取文章列表处理器
pub async fn list_posts_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let posts = state.post_service.list_posts().await?;
    Ok(Json(posts))
}

// 获取单篇文章处理器
pub async fn get_post_handler(
    State(state): State<AppState>,
    Path(id_or_slug): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    // 尝试解析为UUID
    let post = match Uuid::parse_str(&id_or_slug) {
        Ok(id) => state.post_service.get_post_by_id(id).await?,
        Err(_) => state.post_service.get_post_by_slug(&id_or_slug).await?,
    };
    Ok(Json(post))
}

// 更新文章处理器
pub async fn update_post_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePostPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let update_post = state.post_service.update_post(id, payload).await?;
    Ok(Json(update_post))
}

// 删除文章处理器
pub async fn delete_post_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state.post_service.delete_post(id).await?;
    Ok(StatusCode::NO_CONTENT)  // 成功删除返回 204 NO CONTENT
}
