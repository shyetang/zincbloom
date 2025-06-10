use crate::models::{Permission, Role, User, UserRegistrationPayload};
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
    async fn create_user(
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
    async fn create_user(
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
}
