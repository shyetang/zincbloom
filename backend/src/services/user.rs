use crate::dtos::user::{
    ChangePasswordPayload, CreateUserPayload, ResetUserPasswordPayload, UpdateUserPayload,
    UserStatsResponse,
};
use crate::utils::{hash_password, validate_password_strength, verify_password};
use crate::{dtos::user::UpdateProfilePayload, models::UserPublic, repositories::UserRepository};
use anyhow::{Result, anyhow};
use sqlx;
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
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
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
    pub async fn update_my_profile(
        &self,
        user_id: Uuid,
        payload: UpdateProfilePayload,
    ) -> Result<UserPublic> {
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
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("无法找到当前用户"))?;

        // 验证当前密码是否正确
        if !verify_password(&user.hashed_password, &payload.current_password)? {
            return Err(anyhow!("当前密码不正确"));
        }

        // 哈希新密码并更新到数据库
        let new_hashed_password = hash_password(&payload.new_password)?;
        self.user_repo
            .update_password(user_id, &new_hashed_password)
            .await?;

        // 使该用户所有其他会话失效
        self.user_repo
            .delete_all_refresh_token_for_user(user_id)
            .await?;

        tracing::info!("用户 {} 的密码已更改，所有会话已失效。", user_id);

        Ok(())
    }

    // === 管理员功能 ===

    // 管理员创建用户
    pub async fn admin_create_user(&self, payload: CreateUserPayload) -> Result<UserPublic> {
        // 验证密码匹配
        if payload.password != payload.confirm_password {
            return Err(anyhow!("密码与确认密码不匹配"));
        }

        // 验证密码强度
        validate_password_strength(&payload.password)?;

        // 检查用户名和邮箱是否已存在
        if self
            .user_repo
            .find_by_username(&payload.username)
            .await?
            .is_some()
        {
            return Err(anyhow!("用户名 '{}' 已被占用", payload.username));
        }

        if self
            .user_repo
            .find_by_email(&payload.email)
            .await?
            .is_some()
        {
            return Err(anyhow!("邮箱 '{}' 已被占用", payload.email));
        }

        // 哈希密码
        let hashed_password = hash_password(&payload.password)?;

        // 创建用户（注意：这里需要在repository中添加创建方法）
        let user = self
            .user_repo
            .create_user(
                &payload.username,
                &payload.email,
                &hashed_password,
                true, // email_verified_at: 管理员创建的用户默认邮箱已验证
            )
            .await?;

        let roles = self.user_repo.get_user_roles(user.id).await?;

        Ok(UserPublic {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            roles: roles.into_iter().map(|r| r.name).collect(),
        })
    }

    // 管理员更新用户
    pub async fn admin_update_user(
        &self,
        user_id: Uuid,
        payload: UpdateUserPayload,
    ) -> Result<UserPublic> {
        // 检查用户是否存在
        let _existing_user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 如果要更新密码，验证密码匹配和强度
        if let (Some(password), Some(confirm_password)) =
            (&payload.password, &payload.confirm_password)
        {
            if password != confirm_password {
                return Err(anyhow!("密码与确认密码不匹配"));
            }
            validate_password_strength(password)?;
        }

        // 检查用户名和邮箱冲突
        if let Some(new_username) = &payload.username {
            if let Some(other_user) = self.user_repo.find_by_username(new_username).await? {
                if other_user.id != user_id {
                    return Err(anyhow!("用户名 '{}' 已被占用", new_username));
                }
            }
        }

        if let Some(new_email) = &payload.email {
            if let Some(other_user) = self.user_repo.find_by_email(new_email).await? {
                if other_user.id != user_id {
                    return Err(anyhow!("邮箱 '{}' 已被占用", new_email));
                }
            }
        }

        // 构建更新载荷
        let update_payload = UpdateProfilePayload {
            username: payload.username.clone(),
            email: payload.email.clone(),
        };

        // 更新基本信息
        let updated_user = self.user_repo.update(user_id, &update_payload).await?;

        // 如果有密码更新，单独处理
        if let Some(password) = &payload.password {
            let hashed_password = hash_password(password)?;
            self.user_repo
                .update_password(user_id, &hashed_password)
                .await?;
            // 使该用户所有会话失效
            self.user_repo
                .delete_all_refresh_token_for_user(user_id)
                .await?;
        }

        let roles = self.user_repo.get_user_roles(updated_user.id).await?;

        Ok(UserPublic {
            id: updated_user.id,
            username: updated_user.username,
            email: updated_user.email,
            created_at: updated_user.created_at,
            roles: roles.into_iter().map(|r| r.name).collect(),
        })
    }

    // 管理员删除用户
    pub async fn admin_delete_user(&self, user_id: Uuid) -> Result<()> {
        // 检查用户是否存在
        let _user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 删除用户（这会级联删除相关的角色分配、刷新令牌等）
        self.user_repo.delete(user_id).await?;

        tracing::info!("管理员删除了用户 {}", user_id);
        Ok(())
    }

    // 管理员重置用户密码
    pub async fn admin_reset_user_password(
        &self,
        user_id: Uuid,
        payload: ResetUserPasswordPayload,
    ) -> Result<()> {
        // 验证密码匹配
        if payload.new_password != payload.confirm_password {
            return Err(anyhow!("新密码与确认密码不匹配"));
        }

        // 验证密码强度
        validate_password_strength(&payload.new_password)?;

        // 检查用户是否存在
        let _user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 哈希新密码并更新
        let hashed_password = hash_password(&payload.new_password)?;
        self.user_repo
            .update_password(user_id, &hashed_password)
            .await?;

        // 使该用户所有会话失效
        self.user_repo
            .delete_all_refresh_token_for_user(user_id)
            .await?;

        tracing::info!("管理员重置了用户 {} 的密码", user_id);
        Ok(())
    }

    // 管理员获取用户详情
    pub async fn admin_get_user(&self, user_id: Uuid) -> Result<UserPublic> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        let roles = self.user_repo.get_user_roles(user.id).await?;

        Ok(UserPublic {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            roles: roles.into_iter().map(|r| r.name).collect(),
        })
    }

    // 获取用户统计数据
    pub async fn get_user_stats(&self, user_id: Uuid) -> Result<UserStatsResponse> {
        let pool = self.user_repo.get_pool();

        // 获取用户自己的文章统计
        let user_posts_result = sqlx::query!(
            "SELECT 
                COUNT(*) as total_posts,
                COUNT(CASE WHEN published_at IS NOT NULL THEN 1 END) as published_posts,
                COUNT(CASE WHEN published_at IS NULL THEN 1 END) as draft_posts
             FROM posts WHERE author_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await?;

        // 获取全局分类和标签数量
        let categories_result = sqlx::query!("SELECT COUNT(*) as count FROM categories")
            .fetch_one(pool)
            .await?;

        let tags_result = sqlx::query!("SELECT COUNT(*) as count FROM tags")
            .fetch_one(pool)
            .await?;

        Ok(UserStatsResponse {
            total_posts: user_posts_result.total_posts.unwrap_or(0),
            published_posts: user_posts_result.published_posts.unwrap_or(0),
            draft_posts: user_posts_result.draft_posts.unwrap_or(0),
            total_categories: categories_result.count.unwrap_or(0),
            total_tags: tags_result.count.unwrap_or(0),
        })
    }
}
