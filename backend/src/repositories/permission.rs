use crate::models::Permission;
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    // 创建一个权限
    async fn create(&self, name: &str, description: Option<&str>) -> Result<Permission>;
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
    async fn create(&self, name: &str, description: Option<&str>) -> Result<Permission> {
        let permission_id = Uuid::new_v4();
        let permission = sqlx::query_as!(
            Permission,
            r#"
            insert into permissions (id, name, description) 
            values ($1,$2,$3)
            returning id,name,description,created_at,updated_at
            "#,
            permission_id,
            name,
            description
        )
            .fetch_one(&self.pool)
            .await
            .context(format!("创建权限 {} 失败",name))?;
        
        Ok(permission)
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