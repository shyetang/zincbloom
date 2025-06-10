use serde::{Deserialize, Serialize};
use uuid::Uuid;

// 用于"设置用户角色"接口的请求体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetUserRolesPayload {
    // 包含一个角色ID的列表
    // 如果列表为空，则意为清除该用户的所有角色
    pub role_ids: Vec<Uuid>,
}

// 用于设置角色权限接口的请求体

pub struct SetRolePermissionsPayload {
    pub permission_ids: Vec<Uuid>,
}
