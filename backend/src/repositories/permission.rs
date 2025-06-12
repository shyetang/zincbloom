use crate::dtos::admin::{CreatePermissionPayload, UpdatePermissionPayload};
use crate::models::Permission;
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    // 通过id查询权限
    async fn find_by_id(&self, permission_id: Uuid) -> Result<Option<Permission>>;
    // 创建一个权限
    async fn create(&self, payload: &CreatePermissionPayload) -> Result<Permission>;
    // 更新权限
    async fn update(&self, permission_id: Uuid, payload: &UpdatePermissionPayload) -> Result<Permission>;
    // 删除权限
    async fn delete(&self, permission_id: Uuid) -> Result<()>;
    // 列出所有权限
    async fn list(&self) -> Result<Vec<Permission>>;
}

#[derive(Clone)]
pub struct PostgresPermissionRepository {
    pool: PgPool,
}

impl PostgresPermissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl PermissionRepository for PostgresPermissionRepository {
    async fn find_by_id(&self, permission_id: Uuid) -> Result<Option<Permission>> {
        let permission = sqlx::query_as!(
            Permission,
            r#"
            select id,name,description,created_at,updated_at
            from permissions
            where id = $1
            "#,
            permission_id
        )
            .fetch_optional(&self.pool)
            .await?;
        Ok(permission)
    }

    async fn create(&self, payload: &CreatePermissionPayload) -> Result<Permission> {
        let permission_id = Uuid::new_v4();
        let permission = sqlx::query_as!(
            Permission,
            r#"
            insert into permissions (id, name, description) 
            values ($1,$2,$3)
            returning id,name,description,created_at,updated_at
            "#,
            permission_id,
            payload.name,
            payload.description
        )
            .fetch_one(&self.pool)
            .await
            .context(format!("创建权限 {} 失败", payload.name))?;

        Ok(permission)
    }

    async fn update(&self, permission_id: Uuid, payload: &UpdatePermissionPayload) -> Result<Permission> {
        let mut permission = self.find_by_id(permission_id).await?
            .ok_or_else(|| anyhow::anyhow!("未找到ID为 {} 的权限", permission_id))?;

        if let Some(name) = &payload.name {
            permission.name = name.clone();
        }
        if let Some(description) = &payload.description {
            permission.description = Some(description.clone());
        }
        let updated_permission = sqlx::query_as!(
            Permission,
            r#"
            update permissions
            set name = $1,description = $2,updated_at = now()
            where id = $3
            returning *
            "#,
            permission.name,
            permission.description,
            permission_id
        )
            .fetch_one(&self.pool)
            .await?;
        Ok(updated_permission)
    }

    async fn delete(&self, permission_id: Uuid) -> Result<()> {
        let result = sqlx::query!("DELETE FROM permissions WHERE id = $1", permission_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("尝试删除权限失败：未找到ID为 {} 的权限", permission_id));
        }
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Permission>> {
        let permissions = sqlx::query_as!(
            Permission,
            r#"
            select id,name,description,created_at,updated_at
            from permissions
            order by name
            "#
        )
            .fetch_all(&self.pool)
            .await
            .context("获取权限列表失败")?;

        Ok(permissions)
    }
}
