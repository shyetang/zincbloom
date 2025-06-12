use crate::dtos::user::ChangePasswordPayload;
use crate::utils::{hash_password, validate_password_strength, verify_password};
use crate::{
    dtos::user::UpdateProfilePayload,
    models::UserPublic,
    repositories::UserRepository,
};
use anyhow::{anyhow, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    // 获取当前用户的公开信息
    pub async fn get_my_profile(&self, user_id: Uuid) -> Result<UserPublic> {
        let user = self.user_repo.find_by_id(user_id).await?
            .ok_or_else(|| anyhow!("无法找到当前用户的信息"))?;

        let roles = self.user_repo.get_user_roles(user.id).await?;

        Ok(UserPublic {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            roles: roles.into_iter().map(|r| r.name).collect(),
        })
    }

    // 更新当前用户的个人资料
    pub async fn update_my_profile(&self, user_id: Uuid, payload: UpdateProfilePayload) -> Result<UserPublic> {
        // 业务逻辑检查：如果要更新用户名或邮箱，检查新的值是否已被其他用户占用
        if let Some(new_username) = &payload.username {
            if let Some(existing_user) = self.user_repo.find_by_username(new_username).await? {
                if existing_user.id != user_id {
                    return Err(anyhow!("用户名 '{}' 已被占用", new_username));
                }
            }
        }
        if let Some(new_email) = &payload.email {
            if let Some(existing_user) = self.user_repo.find_by_email(new_email).await? {
                if existing_user.id != user_id {
                    return Err(anyhow!("邮箱 '{}' 已被占用", new_email));
                }
            }
        }

        let updated_user = self.user_repo.update(user_id, &payload).await?;
        let roles = self.user_repo.get_user_roles(updated_user.id).await?;

        Ok(UserPublic {
            id: updated_user.id,
            username: updated_user.username,
            email: updated_user.email,
            created_at: updated_user.created_at,
            roles: roles.into_iter().map(|r| r.name).collect(),
        })
    }

    // 删除当前用户的账户
    pub async fn delete_my_account(&self, user_id: Uuid) -> Result<()> {
        // 在这里可以添加其他逻辑，比如记录删除日志、向其他服务发送通知等
        // 目前，我们直接调用 repository 删除用户
        self.user_repo.delete(user_id).await
    }

    // 修改密码
    pub async fn change_my_password(
        &self,
        user_id: Uuid,
        payload: ChangePasswordPayload,
    ) -> Result<()> {
        // 验证新密码与确认密码是否一致
        if payload.new_password != payload.confirm_password {
            return Err(anyhow!("新密码与确认密码不匹配"));
        }
        // 验证新密码是否符合密码强度策略
        validate_password_strength(&payload.new_password)?;
        // 获取当前用户的密码哈希
        let user = self.user_repo.find_by_id(user_id).await?
            .ok_or_else(|| anyhow!("无法找到当前用户"))?;

        // 验证当前密码是否正确
        if !verify_password(&user.hashed_password, &payload.current_password)? {
            return Err(anyhow!("当前密码不正确"));
        }

        // 哈希新密码并更新到数据库
        let new_hashed_password = hash_password(&payload.new_password)?;
        self.user_repo.update_password(user_id, &new_hashed_password).await?;

        // 使该用户所有其他会话失效
        self.user_repo.delete_all_refresh_token_for_user(user_id).await?;

        tracing::info!("用户 {} 的密码已更改，所有会话已失效。", user_id);

        Ok(())
    }
}