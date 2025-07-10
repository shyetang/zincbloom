use serde::Deserialize;
use serde::Serialize;

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

// 管理员创建用户的请求体
#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

// 管理员更新用户的请求体
#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>,
}

// 管理员重置用户密码的请求体
#[derive(Debug, Deserialize)]
pub struct ResetUserPasswordPayload {
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStatsResponse {
    pub total_posts: i64,
    pub published_posts: i64,
    pub draft_posts: i64,
    pub total_categories: i64,
    pub total_tags: i64,
}
