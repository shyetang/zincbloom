use crate::dtos::admin::{CreatePermissionPayload, CreateRolePayload, UpdatePermissionPayload, UpdateRolePayload};
use crate::models::{Permission, Role, UserPublic};
use crate::repositories::{PermissionRepository, RoleRepository, UserRepository};
use anyhow::{Context, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AdminService {
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>,
    permission_repo: Arc<dyn PermissionRepository>,
}

impl AdminService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        role_repo: Arc<dyn RoleRepository>,
        permission_repo: Arc<dyn PermissionRepository>,
    ) -> Self {
        Self {
            user_repo,
            role_repo,
            permission_repo,
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
    pub async fn set_role_permissions(&self, role_id: Uuid, permission_ids: &[Uuid]) -> Result<()> {
        // todo - 检查 permission_ids 是否都在数据库中真实存在

        self.role_repo
            .assign_permissions_to_role(role_id, permission_ids)
            .await
            .context(format!("为角色 {} 设置权限失败", role_id))
    }
    // 获取所有用户的公开信息列表
    pub async fn list_users(&self) -> Result<Vec<UserPublic>> {
        self.user_repo.list_with_roles().await
    }

    // 获取所有角色的列表
    pub async fn list_roles(&self) -> Result<Vec<Role>> {
        self.role_repo.list().await
    }

    // 创建角色
    pub async fn create_role(&self, payload: CreateRolePayload) -> Result<Role> {
        self.role_repo.create(&payload.name, payload.description.as_deref()).await
    }
    // 更新角色
    pub async fn update_role(&self, role_id: Uuid, payload: UpdateRolePayload) -> Result<Role> {
        self.role_repo.update(role_id, &payload).await
    }

    // 删除角色
    pub async fn delete_role(&self, role_id: Uuid) -> Result<()> {
        self.role_repo.delete(role_id).await
    }
    // 获取所有权限的列表
    pub async fn list_permissions(&self) -> Result<Vec<Permission>> {
        self.permission_repo.list().await
    }
    // 创建权限
    pub async fn create_permission(&self, payload: CreatePermissionPayload) -> Result<Permission> {
        self.permission_repo.create(&payload).await
    }
    // 修改权限
    pub async fn update_permission(&self, permission_id: Uuid, payload: UpdatePermissionPayload) -> Result<Permission> {
        self.permission_repo.update(permission_id, &payload).await
    }
    // 删除权限
    pub async fn delete_permission(&self, permission_id: Uuid) -> Result<()> {
        self.permission_repo.delete(permission_id).await
    }
}
