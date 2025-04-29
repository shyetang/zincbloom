use anyhow::Error as AnyhowError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use config::ConfigError;
use serde_json::json;

// 这个结构体用于在 handler 层捕获 anyhow::Error 并转换为响应
pub struct ApiError(AnyhowError);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // 记录详细的错误链和上下文到服务器日志
        // 使用anyhow的{:?}的格式化
        tracing::error!("API层捕获到错误: {:?}", self.0);

        // --- 尝试识别特定错误类型 (可选，增加复杂度) ---
        // 例如，检查是否是未找到记录的错误
        // 注意：downcast 需要知道原始错误类型，或者检查 anyhow::Error 的 Display 输出
        // if let Some(app_level_not_found) = self.0.downcast_ref::<YourNotFoundErrorType>() {
        //     // ... 返回 404 ...
        // }
        // 检查是否由 anyhow::bail!("未找到...") 产生
        let error_string = format!("{}", self.0); // 获取Display输出
        if error_string.contains("未找到") {
            return (StatusCode::NOT_FOUND, Json(json!({"error": error_string}))).into_response();
        }
        // 检查是否唯一约束冲突（更难，需要 downcast sqlx::Error 然后检查 code)
        // if let Some(_db_err)=self.0.downcast_ref::<sqlx::Error>() {
        // if let sqlx::Error::Database(dbe) = db_err{
        //     if dbe.code() == Some(std::borrow::Cow::Borrowed("23505")) {  // PostgreSQL unique violation
        //         let constraint = dbe.constraint().unwrap_or("未知约束");
        //         return (StatusCode::BAD_REQUEST, Json(json!({"error": format!("记录已存在 (违反约束：{}",constraint)}))).into_response();
        //     }
        // }
        if let Some(sqlx::Error::Database(dbe)) = self.0.downcast_ref::<sqlx::Error>() {
            if dbe.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                let constraint = dbe.constraint().unwrap_or("未知约束");
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": format!("记录已存在 (违反约束：{}",constraint)})),
                )
                    .into_response();
            }
        }

        // --- 默认返回通用 500 错误 ---
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        let body = Json(json!({
            "error": "内部服务错误"  // 对客户端隐藏具体错误细节
        }));
        (status, body).into_response()
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
