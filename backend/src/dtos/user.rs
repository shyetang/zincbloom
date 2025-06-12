use serde::Deserialize;

// 用于“更新用户个人资料”接口的请求体
// 所有字段都是可选的，允许用户只更新部分信息
#[derive(Debug, Deserialize, Default)]
pub struct UpdateProfilePayload {
    pub username: Option<String>,
    pub email: Option<String>,
}

// 用于“修改密码”接口的请求体
#[derive(Debug, Deserialize)]
pub struct ChangePasswordPayload {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}