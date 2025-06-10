use serde::{Deserialize, Serialize};
use crate::models::UserPublic;

// 用于 refresh 和 logout 接口的请求体
// 两者都需要 refresh_token
#[derive(Debug,Deserialize)]
pub struct RefreshTokenPayload {
    pub refresh_token: String,
}

// 用于登录和刷新token成功后返回给客户端的响应体
#[derive(Debug,Serialize)]
pub struct LoginResponsePayload {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserPublic,
}