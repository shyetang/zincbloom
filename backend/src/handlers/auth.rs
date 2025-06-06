use axum::{extract::{State,Json}, http::StatusCode};
use anyhow::Result;
use axum::response::{IntoResponse};
use serde_json::json;
use crate::api_error::ApiError;
use crate::handlers::AppState;
use crate::models::{UserLoginPayload, UserRegistrationPayload};

/// 用户注册处理器
pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<UserRegistrationPayload>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("接收到用户注册请求：{}",payload.username);
    let user_public = state.auth_service.register_user(payload).await?;
    
    Ok((StatusCode::CREATED,Json(user_public)))
}

/// 用户登录处理器
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<UserLoginPayload>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("接收到用户登录请求: {}", payload.username);
    let (token, user_public) = state.auth_service.login_user(payload).await?;
    Ok(Json(json!({
        "token": token,
        "user": user_public
    })))
}