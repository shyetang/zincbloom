use crate::dtos::{UpdateProfilePayload, UserRegistrationPayload};
use crate::models::{Permission, Role, User, UserPublic};
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

// 定义 UserRepository trait，抽象用户数据的数据库操作
#[async_trait]
pub trait UserRepository: Send + Sync {
    // Send + Sync 使得 trait 对象可以在多线程中安全共享
    // 创建新用户
    async fn create(
        &self,
        payload: &UserRegistrationPayload,
        hashed_password: &str,
    ) -> Result<User>;
    // 根据用户名查找用户
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    // 根据邮箱查找用户
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    // 根据用户id查找用户
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>>;
    // 给用户分配角色
    async fn assign_roles_to_user(&self, user_id: Uuid, role_ids: &[Uuid]) -> Result<()>;
    // 从用户移除角色
    async fn remove_role_from_user(&self, user_id: Uuid, role_id: Uuid) -> Result<()>;
    // 设置用户的角色列表（先清空后添加）
    async fn set_user_roles(&self, user_id: Uuid, role_ids: &[Uuid]) -> Result<()>;
    // 获取用户拥有的所有角色
    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>>;
    // 获取用户拥有的所有权限(通过其角色间接获得）
    async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<Permission>>;
    // 存储 Refresh Token 哈希到数据库
    async fn store_refresh_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<()>;
    // 通过 Refresh Token 哈希查找关联的用户。如果 token 过期，则不返回任何内容
    async fn find_user_by_refresh_token(&self, token_hash: &str) -> Result<Option<User>>;
    // 删除指定的 Refresh Token 哈希
    async fn delete_refresh_token(&self, token_hash: &str) -> Result<()>;
    // 列出所有用户
    async fn list(&self) -> Result<Vec<User>>;
    // 一次性获取所有用户及其角色列表的方法
    async fn list_with_roles(&self) -> Result<Vec<UserPublic>>;
    // 更新用户
    async fn update(&self, user_id: Uuid, payload: &UpdateProfilePayload) -> Result<User>;
    // 删除用户
    async fn delete(&self, user_id: Uuid) -> Result<()>;

    // 更新密码
    async fn update_password(&self, user_id: Uuid, new_hashed_password: &str) -> Result<()>;
    // 为用户删除所有的 refresh token
    async fn delete_all_refresh_token_for_user(&self, user_id: Uuid) -> Result<()>;
}

// UserRepository 的 PostgreSQL 具体实现
#[derive(Clone)] // Clone 以便在 Axum AppState 中共享
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    // 构造函数
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(
        &self,
        payload: &UserRegistrationPayload,
        hashed_password: &str,
    ) -> Result<User> {
        let user_id = Uuid::new_v4();
        let user = sqlx::query_as!(
            User,
            r#"
            insert into users (id, username, email, hashed_password)
            VALUES ($1,$2,$3,$4)
            returning id,username,email,hashed_password,created_at,updated_at
            "#,
            user_id,
            payload.username,
            payload.email,
            hashed_password
        )
            .fetch_one(&self.pool)
            .await
            .context("创建User失败")?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            select id,username,email,hashed_password,created_at,updated_at
            from users
            where username = $1
            "#,
            username
        )
            .fetch_optional(&self.pool)
            .await
            .context(format!("根据 {} 查找用户失败", username))?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            select id,username,email,hashed_password,created_at,updated_at
            from users
            where email = $1
            "#,
            email
        )
            .fetch_optional(&self.pool)
            .await
            .context(format!("根据 {} 查找用户失败 ", email))?;

        Ok(user)
    }

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            select id,username,email,hashed_password,created_at,updated_at
            from users
            where id = $1
            "#,
            user_id
        )
            .fetch_optional(&self.pool)
            .await
            .context(format!("根据用户id {} 查找用户失败 ", user_id))?;

        Ok(user)
    }

    async fn assign_roles_to_user(&self, user_id: Uuid, role_ids: &[Uuid]) -> Result<()> {
        if role_ids.is_empty() {
            return Ok(());
        }
        sqlx::query!(
            r#"insert into user_roles (user_id, role_id) 
            select $1,role_id from unnest($2::uuid[]) as t(role_id)
            on conflict (user_id,role_id) do nothing 
            "#,
            user_id,
            role_ids,
        )
            .execute(&self.pool)
            .await
            .context(format!("给用户id {} 分配角色失败", user_id))?;
        Ok(())
    }

    async fn remove_role_from_user(&self, user_id: Uuid, role_id: Uuid) -> Result<()> {
        sqlx::query!(
            "delete from user_roles where user_id = $1 and role_id = $2",
            user_id,
            role_id
        )
            .execute(&self.pool)
            .await
            .context(format!("为用户id {} 移除 角色id {} 失败", user_id, role_id))?;

        Ok(())
    }

    async fn set_user_roles(&self, user_id: Uuid, role_ids: &[Uuid]) -> Result<()> {
        // 在单个事务中执行“先删除后添加”的操作，确保原子性
        let mut tx = self.pool.begin().await.context("开启数据库事务失败")?;
        // 1. 删除该用户所有已存在的角色
        sqlx::query!("delete from user_roles where user_id = $1", user_id)
            .execute(&mut *tx)
            .await
            .context("删除用户旧角色失败")?;

        // 2. 如果提供了新的角色id列表，则批量插入新的角色
        if !role_ids.is_empty() {
            sqlx::query!(
                r#"
                insert into user_roles (user_id, role_id) 
                select $1,role_id from unnest($2::uuid[]) as t(role_id)
                "#,
                user_id,
                role_ids
            )
                .execute(&mut *tx)
                .await
                .context("插入新用户角色失败")?;
        }

        // 3 提交事务
        tx.commit().await.context("插入新角色失败")?;

        Ok(())
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>> {
        let roles = sqlx::query_as!(
            Role,
            r#"
            select r.id,r.name,r.description,r.created_at,r.updated_at
            from roles r 
            inner join user_roles ur on r.id = ur.role_id
            where ur.user_id = $1
            order by r.name -- 按角色名称排序
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await
            .context("获取用户ID {} 所有角色失败")?;

        Ok(roles)
    }

    async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<Permission>> {
        let permissions = sqlx::query_as!(
            Permission,
            r#"
            select distinct p.id,p.name,p.description,p.created_at,p.updated_at
            from permissions p 
            inner join role_permissions rp on p.id = rp.permission_id
            inner join user_roles ur on rp.role_id = ur.role_id
            where ur.user_id = $1
            order by p.name -- 按权限名称排序
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await
            .context(format!("获取用户ID {} 的所有权限失败", user_id))?;

        Ok(permissions)
    }

    async fn store_refresh_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query!(
            "insert into refresh_tokens (token_hash, user_id, expires_at) VALUES ($1,$2,$3)",
            token_hash,
            user_id,
            expires_at
        )
            .execute(&self.pool)
            .await
            .context("存储用户刷新令牌失败")?;

        Ok(())
    }

    async fn find_user_by_refresh_token(&self, token_hash: &str) -> Result<Option<User>> {
        // 查找用户,  并检查令牌是否过期
        let user = sqlx::query_as!(
            User,
            r#"
            select u.id,u.username,u.email,u.hashed_password,u.created_at,u.updated_at
            from users u
            inner join refresh_tokens rt on u.id = rt.user_id
            where rt.token_hash = $1 and rt.expires_at > now()
            "#,
            token_hash
        )
            .fetch_optional(&self.pool)
            .await
            .context("查找用户刷新令牌失败")?;
        Ok(user)
    }

    async fn delete_refresh_token(&self, token_hash: &str) -> Result<()> {
        let result = sqlx::query!(
            "delete from refresh_tokens where token_hash = $1",
            token_hash
        )
            .execute(&self.pool)
            .await
            .context("删除用户刷新令牌失败")?;

        // 如果没有行被删除，可能意味着 token 已经被撤销或不存在，这通常不是一个需要向上传播的错误
        if result.rows_affected() == 0 {
            tracing::warn!("尝试删除一个不存在或已过期的 refresh token: {}", token_hash);
        }

        Ok(())
    }

    async fn list(&self) -> Result<Vec<User>> {
        sqlx::query_as!(
            User,
            "select id,username,email,hashed_password, created_at,updated_at from users order by username"
        )
            .fetch_all(&self.pool)
            .await
            .context("数据库层查询用户列表失败")
    }

    async fn list_with_roles(&self) -> Result<Vec<UserPublic>> {
        let user_with_roles = sqlx::query_as!(
            UserPublic,
            r#"
            select
                u.id,
                u.username,
                u.email,
                u.created_at,
                -- 使用 coalesce 和 array_agg 来聚合角色名称
                -- array_agg(r.name) 会将每个用户关联的所有角色名聚合成一个数组
                -- filter (where r.name is not null) 用于处理没有角色的用户，避免数组中出现 NULL 元素
                -- coalesce(..., '{}') 确保即使用户没有任何角色（LEFT JOIN结果为NULL），也返回一个空的数组 '{}' 而不是 NULL
                -- as "roles!: Vec<String>" 是 sqlx 的语法，用于将SQL数组映射到 Rust 的 Vec<String>
                -- `!` 表示我们断言这个字段不会是 NULL（因为 COALESCE 保证了这一点）
                coalesce(
                    array_agg(r.name) filter ( where r.name is not null),
                    '{}'
                ) as "roles!: Vec<String>"
            from
                users u
            left join user_roles ur on u.id = ur.user_id
            left join roles r on ur.role_id = r.id
            group by u.id, u.username, u.email, u.created_at
            order by u.username
            "#,
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(user_with_roles)
    }

    async fn update(&self, user_id: Uuid, payload: &UpdateProfilePayload) -> Result<User> {
        let mut user = self.find_by_id(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("未找到ID为 {} 的用户", user_id))?;

        if let Some(username) = &payload.username {
            user.username = username.clone();
        }
        if let Some(email) = &payload.email {
            user.email = email.clone();
        }

        let updated_user = sqlx::query_as!(
            User,
            r#"
            update users
            set username = $1, email = $2, updated_at = NOW()
            where id = $3
            returning id, username, email, hashed_password, created_at, updated_at
            "#,
            user.username,
            user.email,
            user_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(updated_user)
    }

    async fn delete(&self, user_id: Uuid) -> Result<()> {
        // ON DELETE CASCADE 约束会自动处理 user_roles 和 refresh_tokens 表中的关联记录
        // ON DELETE SET NULL 会自动处理 posts 表中的 author_id
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("尝试删除用户失败：未找到ID为 {} 的用户", user_id));
        }
        Ok(())
    }

    async fn update_password(&self, user_id: Uuid, new_hashed_password: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET hashed_password = $1, updated_at = NOW() WHERE id = $2",
            new_hashed_password,
            user_id
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_all_refresh_token_for_user(&self, user_id: Uuid) -> Result<()> {
        // 删除该用户所有已存在的 refresh token，强制所有设备重新登录
        sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
