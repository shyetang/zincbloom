use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::api_error::ApiError;
use crate::auth::AuthUser;
use crate::dtos::category::{CreateCategoryPayload, UpdateCategoryPayload};
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
