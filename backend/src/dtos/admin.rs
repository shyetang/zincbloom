use serde::{Deserialize, Serialize};
use uuid::Uuid;

// 用于用户注册API的请求体结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRegistrationPayload {
    pub username: String,
    pub email: String,
    pub password: String, // 注册时提交的明文密码 (仅用于传输，后续会被哈希)
}

// 用于用户登录API的请求体结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLoginPayload {
    pub username: String, // 登录用户名或邮箱
    pub password: String, // 登录时提交的明文密码
}

// 用于“创建角色”接口的请求体
#[derive(Debug, Deserialize)]
pub struct CreateRolePayload {
    pub name: String,
    pub description: Option<String>,
}
// 用于“更新角色”接口的请求体
#[derive(Debug, Deserialize)]
pub struct UpdateRolePayload {
    pub name: Option<String>,
    pub description: Option<String>,
}

// 用于“创建权限”接口的请求体
#[derive(Debug, Deserialize)]
pub struct CreatePermissionPayload {
    pub name: String,
    pub description: Option<String>,
}

// 用于“更新权限”接口的请求体
#[derive(Debug, Deserialize)]
pub struct UpdatePermissionPayload {
    pub name: Option<String>,
    pub description: Option<String>,
}

// 用于"设置用户角色"接口的请求体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetUserRolesPayload {
    // 包含一个角色ID的列表
    // 如果列表为空，则意为清除该用户的所有角色
    pub role_ids: Vec<Uuid>,
}

// 用于设置角色权限接口的请求体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetRolePermissionsPayload {
    pub permission_ids: Vec<Uuid>,
}

