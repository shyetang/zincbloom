use crate::repositories::{RoleRepository, UserRepository};
use anyhow::{Context, Result};
use std::sync::Arc;
use uuid::Uuid;
#[derive(Clone)]
pub struct AdminService {
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>,
}

impl AdminService {
    pub fn new(user_repo: Arc<dyn UserRepository>, role_repo: Arc<dyn RoleRepository>) -> Self {
        Self {
            user_repo,
            role_repo,
        }
    }

    // 为指定用户设置角色列表的业务逻辑
    // target_user_id: 要被操作的用户ID
    // role_ids: 要为该用户设置的角色ID列表
    pub async fn set_user_roles(&self, target_user_id: Uuid, role_ids: &[Uuid]) -> Result<()> {
        // todo - 检查 `role_ids` 中的每个ID是否都在数据库中真实存在

        self.user_repo
            .set_user_roles(target_user_id, role_ids)
            .await
            .context(format!("为用户 {} 设置角色失败", target_user_id))
    }
    
    // 为角色设置权限
    pub async fn set_role_permissions(&self,role_id: Uuid,permission_ids: &[Uuid]) -> Result<()> {
        // todo - 检查 permission_ids 是否都在数据库中真实存在

        self.role_repo
            .assign_permissions_to_role(role_id, permission_ids)
            .await
            .context(format!("为角色 {} 设置权限失败", role_id))
    }
}
