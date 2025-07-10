use anyhow::Error as AnyhowError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use config::ConfigError;
use serde_json::json;

// 这个结构体用于在 handler 层捕获 anyhow::Error 并转换为响应
pub struct ApiError(AnyhowError);

// 辅助函数，用于检查错误链中是否包含特定关键词
fn error_chain_contains(error: &AnyhowError, keyword: &str) -> bool {
    // 检查顶层错误消息
    if format!("{:?}", error).to_lowercase().contains(keyword) {
        return true;
    }
    // 遍历 source 链
    let mut current_source = error.source();
    while let Some(source) = current_source {
        if format!("{:?}", source).to_lowercase().contains(keyword) {
            return true;
        }
        current_source = source.source()
    }
    false
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // 记录完整的错误信息（使用Debug格式）以供调试
        tracing::error!("API层捕获到错误(debug): {:?}", self.0);

        // --- 1. 认证 (401) 和授权 (403) 错误 ---
        if error_chain_contains(&self.0, "bearer token")
            || error_chain_contains(&self.0, "无效或过期的 token")
            || error_chain_contains(&self.0, "无效的 refresh token")
        {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "认证失败或Token无效" })),
            )
                .into_response();
        }

        if error_chain_contains(&self.0, "权限") // 匹配 "权限不足"
            || error_chain_contains(&self.0, "permission")  // 匹配 "permission"
            || error_chain_contains(&self.0, "只能")
        // 匹配 "您只能编辑/删除自己的帖子"
        {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({ "error": format!("{}", self.0) })),
            )
                .into_response();
        }

        // --- 2. 数据库约束和冲突错误 ---
        if let Some(db_error) = self.0.downcast_ref::<sqlx::Error>() {
            if let Some(db_error_inner) = db_error.as_database_error() {
                if db_error_inner.is_unique_violation() {
                    return (
                        StatusCode::CONFLICT, // 409 Conflict 更适合唯一约束
                        Json(json!({ "error": "记录已存在或与现有数据冲突" })),
                    )
                        .into_response();
                }
            }
        }
        // 对 "已存在"、"已被占用" 这类业务逻辑错误使用 409
        if error_chain_contains(&self.0, "已存在")
            || error_chain_contains(&self.0, "已被占用")
            || error_chain_contains(&self.0, "已被注册")
        {
            return (
                StatusCode::CONFLICT,
                Json(json!({ "error": format!("{}", self.0) })),
            )
                .into_response();
        }

        // --- 3. 资源未找到错误 (404) ---
        if error_chain_contains(&self.0, "未找到") || error_chain_contains(&self.0, "not found")
        {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("{}", self.0) })),
            )
                .into_response();
        }

        // --- 4. 业务逻辑验证/格式错误 (400) ---
        if error_chain_contains(&self.0, "无效")
            || error_chain_contains(&self.0, "格式")
            || error_chain_contains(&self.0, "不匹配")
        {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("{}", self.0) })),
            )
                .into_response();
        }

        // --- 5. 配置错误 (500) ---
        if self.0.downcast_ref::<ConfigError>().is_some() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "服务器配置错误"})),
            )
                .into_response();
        }

        // --- 6. 默认返回通用 500 错误 ---
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "内部服务器错误,请稍候再试"})),
        )
            .into_response()
    }
}

// 实现 From<anyhow::Error> for ApiError 以便在 handler 中使用 '?'
impl From<AnyhowError> for ApiError {
    fn from(error: AnyhowError) -> Self {
        ApiError(error)
    }
}

/* impl From<ConfigError> for ApiError {
    fn from(error: ConfigError) -> Self {
        ApiError(error.into())
    }
} */
