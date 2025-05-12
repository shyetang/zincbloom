use anyhow::Error as AnyhowError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use config::ConfigError;
use serde_json::json;
use std::error::Error as StdError;

// 这个结构体用于在 handler 层捕获 anyhow::Error 并转换为响应
pub struct ApiError(AnyhowError);

// 辅助函数，用于检查错误链中是否包含特定关键词
fn error_chain_contains(error: &AnyhowError, keyword: &str) -> bool {
    // 检查顶层错误消息
    if format!("{}", error).to_lowercase().contains(keyword) {
        return true;
    }
    // 遍历 source 链
    let mut current_source = error.source();
    while let Some(source) = current_source {
        if format!("{}", source).to_lowercase().contains(keyword) {
            return true;
        }
        current_source = source.source()
    }
    false
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // 1. 首先记录详细的错误链，便于服务器端调试
        //    self.0 是 anyhow::Error，它的 Debug 实现会包含错误链和上下文
        // 使用anyhow的{:?}的格式化
        tracing::error!("API层捕获到错误(debug): {:?}", self.0);

        let error_display_for_client = format!("{}", self.0); // 获取 Display 输出
        tracing::info!(
            "[DEBUG] ApiError IntoResponse - Top-level error display string: '{}'",
            error_display_for_client
        );

        // 2. 优先处理由 anyhow! 或 bail! 产生的应用层面 “未找到” 错误
        //    这些错误不是数据库层面的 RowNotFound,而是业务逻辑判断的结果
        // 使用辅助函数检查整个错误链
        if error_chain_contains(&self.0, "未找到") || error_chain_contains(&self.0, "not found")
        {
            tracing::info!("[DEBUG] Matched '未找到'/'not found' in error chain, returning 404.");
            // 返回顶层的错误信息，或者一个更通用的 "未找到" 消息
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": error_display_for_client})),
            )
                .into_response();
        }
        tracing::warn!("[DEBUG] Did NOT match '未找到'/'not found'. Proceeding."); // 调试日志

        // 3. 尝试 downcast 到 sqlx::Error 来处理特定的数据库错误
        //    需要检查 self.0.source() 链条，因为 anyhow::Error 可能包装了多层
        let mut root_cause: Option<&(dyn StdError + 'static)> = Some(&*self.0); //&*self.0 获取对内部错误的引用
        while let Some(cause) = root_cause {
            if let Some(sqlx_err) = cause.downcast_ref::<sqlx::Error>() {
                match sqlx_err {
                    sqlx::Error::Database(dbe) => {
                        if dbe.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                            // PostgreSQL unique violation
                            let constraint = dbe.constraint().unwrap_or("未知约束");
                            let user_message =
                                format!("记录已存在或唯一性冲突(约束：{}", constraint);
                            return (
                                StatusCode::BAD_REQUEST,
                                Json(json!({"error": user_message})),
                            )
                                .into_response();
                        }
                        // 可以为其他 dbe.code() 添加处理
                        // 如果是其他数据库错误，下面会作为通用500处理
                        break; // 已找到 sqlx::Error::Database，跳出循环
                    }
                    sqlx::Error::RowNotFound => {
                        // 这个 RowNotFound 是 sqlx 底层返回的，也应该映射为 404
                        return (
                            StatusCode::NOT_FOUND,
                            Json(json!({"error": "请求的资源未找到（数据库）"})),
                        )
                            .into_response();
                    }
                    // 可以为 sqlx::Error::PoolTimedOut 等添加特定处理
                    _ => {
                        // 其他 sqlx 错误，可能归为连接问题或更通用的数据库问题
                        break; // 跳出循环，让它走到下面的通用500
                    }
                }
            }
            root_cause = cause.source(); // 继续查找 source 链
        }

        // 4. 如果是其他类型的已知错误（例如配置错误，如果 ApiError 能包装它）
        if let Some(_config_err) = self.0.downcast_ref::<ConfigError>() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "服务器配置错误"})),
            )
                .into_response();
        }

        // 5. 默认返回通用 500 错误
        // 对于所有未被上面特定逻辑捕获的错误，都视为内部服务器错误
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

impl From<ConfigError> for ApiError {
    fn from(error: ConfigError) -> Self {
        ApiError(error.into()) // 将 config::ConfigError 转换为 anyhow::Error
    }
}
