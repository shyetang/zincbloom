// 创建一个自定义的 Axum Extractor，它负责从请求头中提取 Bearer Token，验证它，
// 如果成功，则将解码后的 Claims 提供给我们的处理器
use crate::api_error::ApiError;
use crate::handlers::AppState;
use crate::services::auth::Claims;
use anyhow::Result;
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

// #[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 从请求头中提取 Authorization: Bearer <token>
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| {
                tracing::warn!("解析 Authorization header 失败: {}", e);
                ApiError::from(anyhow::anyhow!("需要提供Bearer token"))
            })?;

        // 获取 AuthService 实例
        let auth_service = Arc::clone(&state.auth_service);

        // 验证 token
        let token_data = auth_service
            .validate_token(bearer.token())
            .map_err(|e| ApiError::from(e.context("无效的 token")))?;

        // 验证成功，返回包含 Claims 的 AuthUser
        Ok(AuthUser(token_data.claims))
    }
}

// 辅助函数，用于在 handler 中检查权限
// 它不是提取器，而是一个可以在 handler 内部调用的函数
impl AuthUser {
    pub fn user_id(&self) -> Uuid {
        Uuid::parse_str(&self.0.sub).unwrap_or_default()
    }

    pub fn require_permission(&self, permission: &str) -> Result<(), ApiError> {
        if self.0.permissions.contains(&permission.to_string()) {
            Ok(())
        } else {
            tracing::warn!(
                "权限不足： 用户 {} 尝试访问需要 '{}' 权限的操作",
                self.0.username,
                permission
            );
            Err(ApiError::from(anyhow::anyhow!("权限不足")))
        }
    }
}
