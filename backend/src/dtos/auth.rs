use crate::models::UserPublic;
use serde::{Deserialize, Serialize};

// 用于 refresh 和 logout 接口的请求体
// 两者都需要 refresh_token
#[derive(Debug, Deserialize)]
pub struct RefreshTokenPayload {
    pub refresh_token: String,
}

// 用于登录和刷新token成功后返回给客户端的响应体
#[derive(Debug, Serialize)]
pub struct LoginResponsePayload {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserPublic,
}

// 用于验证邮箱
#[derive(Debug, Deserialize)]
pub struct VerifyEmailPayload {
    pub token: String,
}

// 用于忘记密码处理器的请求体
#[derive(Debug, Deserialize)]
pub struct ForgotPasswordPayload {
    pub email: String,
}

// 用于重置密码的处理器
#[derive(Debug, Deserialize)]
pub struct ResetPasswordPayload {
    pub token: String,
    pub new_password: String,
    pub confirm_password: String,
}