// 创建一个自定义的 Axum Extractor，它负责从请求头中提取 Bearer Token，验证它，
// 如果成功，则将解码后的 Claims 提供给我们的处理器
use crate::api_error::ApiError;
use crate::handlers::AppState;
use crate::services::auth::Claims;
use anyhow::Result;
use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);    // 认证用户

/// 用户上下文：游客或已认证用户
#[derive(Debug, Clone)]
pub enum UserContext {
    Guest,                   // 未认证用户（游客）
    Authenticated(AuthUser), // 已认证用户
}

/// 可选认证：用于处理既允许游客又允许认证用户的端点
#[derive(Debug, Clone)]
pub struct OptionalAuth(pub UserContext);

// 从请求头中提取 Authorization: Bearer <token> 并验证 token
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

impl FromRequestParts<AppState> for OptionalAuth {
    type Rejection = ApiError;

    // 从请求头中提取 Authorization: Bearer <token>
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 尝试提取 Authorization header，如果失败则返回 Guest
        let auth_header = parts.extract::<TypedHeader<Authorization<Bearer>>>().await;

        match auth_header {
            Ok(TypedHeader(Authorization(bearer))) => {
                // 有 token，尝试验证
                let auth_service = Arc::clone(&state.auth_service);
                match auth_service.validate_token(bearer.token()) {
                    Ok(token_data) => {
                        // 验证成功，返回已认证用户
                        Ok(OptionalAuth(UserContext::Authenticated(AuthUser(
                            token_data.claims,
                        ))))
                    }
                    Err(_) => {
                        // token 无效，但不报错，视为游客
                        tracing::debug!("无效的token，视为游客访问");
                        Ok(OptionalAuth(UserContext::Guest))
                    }
                }
            }
            Err(_) => {
                // 没有 Authorization header，视为游客
                Ok(OptionalAuth(UserContext::Guest))
            }
        }
    }
}

// 辅助函数，用于在 handler 中检查权限
// 不是提取器，而是一个可以在 handler 内部调用的函数
impl AuthUser {
    // 获取用户ID
    pub fn user_id(&self) -> Uuid {
        Uuid::parse_str(&self.0.sub).unwrap_or_default()
    }

    // 获取用户名
    pub fn username(&self) -> String {
        self.0.username.clone()
    }

    // 检查权限
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

// 用户上下文：游客或已认证用户
impl UserContext {
    /// 检查是否有指定权限（游客自动拥有 post:read_published 权限）
    pub fn has_permission(&self, permission: &str) -> bool {
        match self {
            UserContext::Guest => {
                // 游客自动拥有读取已发布文章的权限
                permission == "post:read_published"
            }
            // 已认证用户
            UserContext::Authenticated(auth_user) => {
                auth_user.0.permissions.contains(&permission.to_string())
            }
        }
    }

    /// 要求指定权限，如果没有则返回错误
    pub fn require_permission(&self, permission: &str) -> Result<(), ApiError> {
        if self.has_permission(permission) {
            Ok(())
        } else {
            match self {
                UserContext::Guest => {
                    tracing::warn!("游客尝试访问需要 '{}' 权限的操作", permission);
                    Err(ApiError::from(anyhow::anyhow!("需要登录才能访问此功能")))
                }
                UserContext::Authenticated(auth_user) => {
                    tracing::warn!(
                        "权限不足： 用户 {} 尝试访问需要 '{}' 权限的操作",
                        auth_user.0.username,
                        permission
                    );
                    Err(ApiError::from(anyhow::anyhow!("权限不足")))
                }
            }
        }
    }

    /// 获取用户ID（仅对已认证用户有效）
    pub fn user_id(&self) -> Option<Uuid> {
        match self {
            UserContext::Guest => None,
            UserContext::Authenticated(auth_user) => Some(auth_user.user_id()),
        }
    }

    /// 获取用户名（仅对已认证用户有效）
    pub fn username(&self) -> Option<String> {
        match self {
            UserContext::Guest => None,
            UserContext::Authenticated(auth_user) => Some(auth_user.username()),
        }
    }

    /// 是否为游客
    pub fn is_guest(&self) -> bool {
        matches!(self, UserContext::Guest)
    }

    /// 是否为已认证用户
    pub fn is_authenticated(&self) -> bool {
        matches!(self, UserContext::Authenticated(_))
    }
}

// 可选认证：用于处理既允许游客又允许认证用户的端点
impl OptionalAuth {
    /// 获取用户上下文
    pub fn context(&self) -> &UserContext {
        &self.0
    }

    /// 方便的权限检查方法
    pub fn has_permission(&self, permission: &str) -> bool {
        self.0.has_permission(permission)
    }

    /// 方便的权限要求方法
    pub fn require_permission(&self, permission: &str) -> Result<(), ApiError> {
        self.0.require_permission(permission)
    }
}
