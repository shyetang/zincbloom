use crate::api_error::ApiError;
use crate::auth::AuthUser;
use crate::dtos::post::{CreatePostPayload, UpdatePostPayload};
use crate::dtos::Pagination;
use crate::handlers::AppState;
use anyhow::Result;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;
// 定义应用状态，包含服务实例
// 使用 Arc 来安全地在多个线程间共享服务实例

// Handler 返回 Result<impl IntoResponse, ApiError>
// ApiError 实现了 From<anyhow::Error>,所以可以在 service 调用后用 '?'
// 创建文章处理器
pub async fn create_post_handler(
    auth_user: AuthUser,                    // 要求认证用户
    State(state): State<AppState>,          // 从状态中提取PostService
    Json(payload): Json<CreatePostPayload>, // 从请求体解析 JSON
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查： 要求 "post:create" 权限
    auth_user.require_permission("post:create")?;

    // 从认证信息中获取作者 ID// 从认证信息中获取作者 ID
    let author_id = auth_user.user_id();

    // 返回 Result<impl IntoResponse, ApiError>
    let post_detail = state.post_service.create_post(author_id, payload).await?; //调用服务层方法
    Ok((StatusCode::CREATED, Json(post_detail))) // 成功返回 201 CREATED 和 JSON 数据
}

// 获取文章列表处理器
pub async fn list_posts_handler(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>, // 使用 Query 提取器获取分页参数
) -> Result<impl IntoResponse, ApiError> {
    // 将提取到的 pagination 参数传递给 service 方法
    let paginated_response = state.post_service.list_posts(pagination).await?;
    Ok(Json(paginated_response))
}

// 获取单篇文章处理器
pub async fn get_post_handler(
    State(state): State<AppState>,
    Path(id_or_slug): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    // 尝试解析为UUID
    let post_detail = match Uuid::parse_str(&id_or_slug) {
        Ok(id) => state.post_service.get_post_by_id(id).await?,
        Err(_) => state.post_service.get_post_by_slug(&id_or_slug).await?,
    };
    Ok(Json(post_detail))
}

// 更新文章处理器
pub async fn update_post_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePostPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let update_post_detail = state.post_service.update_post(id, payload).await?;
    Ok(Json(update_post_detail))
}

// 删除文章处理器
pub async fn delete_post_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state.post_service.delete_post(id).await?;
    Ok(StatusCode::NO_CONTENT) // 成功删除返回 204 NO CONTENT
}
