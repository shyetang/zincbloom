use crate::dtos::user::ChangePasswordPayload;
use crate::{
    api_error::ApiError, auth::AuthUser, dtos::user::UpdateProfilePayload, handlers::AppState,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

// 获取 "我" 的个人资料
pub async fn get_my_profile_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = auth_user.user_id();
    let user_profile = state.user_service.get_my_profile(user_id).await?;
    Ok(Json(user_profile))
}

// 更新 "我" 的个人资料
pub async fn update_my_profile_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateProfilePayload>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = auth_user.user_id();
    let updated_profile = state
        .user_service
        .update_my_profile(user_id, payload)
        .await?;
    Ok(Json(updated_profile))
}

// 删除 "我" 的账户
pub async fn delete_my_account_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = auth_user.user_id();
    state.user_service.delete_my_account(user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// 修改 "我" 的密码
pub async fn change_my_password_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<ChangePasswordPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = auth_user.user_id();
    state
        .user_service
        .change_my_password(user_id, payload)
        .await?;
    // 成功后无需返回内容，204
    Ok(StatusCode::NO_CONTENT)
}

// 获取用户自己的统计数据处理器
pub async fn get_my_stats_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // 用户可以查看自己的统计数据，不需要特殊权限
    let user_id = auth_user.user_id();
    let stats = state.user_service.get_user_stats(user_id).await?;
    Ok(Json(stats))
}
