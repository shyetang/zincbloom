use crate::api_error::ApiError;
use crate::dtos::auth::{ForgotPasswordPayload, ResetPasswordPayload, VerifyEmailPayload};
use crate::dtos::{LoginResponsePayload, RefreshTokenPayload, UserLoginPayload, UserRegistrationPayload};
use crate::handlers::AppState;
use anyhow::Result;
use axum::response::IntoResponse;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

// 用户注册处理器
pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<UserRegistrationPayload>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("接收到用户注册请求：{}", payload.username);
    let user_public = state.auth_service.register_user(payload).await?;

    Ok((StatusCode::CREATED, Json(user_public)))
}

// 用户登录处理器
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<UserLoginPayload>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("接收到用户登录请求: {}", payload.username);

    let (tokens, user_public) = state.auth_service.login_user(payload).await?;

    let response_payload = LoginResponsePayload {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user: user_public,
    };

    Ok(Json(response_payload))
}

// 刷新token
pub async fn refresh_token_handler(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenPayload>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("接收到刷新 Token 请求");

    // 刷新token
    let tokens = state
        .auth_service
        .refresh_access_token(&payload.refresh_token)
        .await?;

    Ok(Json(tokens))
}

// 注销用户
pub async fn logout_handler(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenPayload>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("接收到注销请求");

    state.auth_service.logout(&payload.refresh_token).await?;

    Ok(StatusCode::NO_CONTENT)
}

// 验证邮箱处理器
pub async fn verify_email_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<impl IntoResponse, ApiError> {
    state.auth_service.verify_email(&payload.token).await?;
    Ok(StatusCode::NO_CONTENT)
}

// 忘记密码处理器
pub async fn forgot_password_handler(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordPayload>,
) -> Result<impl IntoResponse, ApiError> {
    state.auth_service.request_password_reset(&payload.email).await?;

    Ok(StatusCode::NO_CONTENT)
}

// 重置密码处理器
pub async fn reset_password_handler(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordPayload>,
) -> Result<impl IntoResponse, ApiError> {
    state.auth_service.reset_password(&payload).await?;
    Ok(StatusCode::NO_CONTENT)
}