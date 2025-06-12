use crate::dtos::admin::UpdateRolePayload;
use crate::models::{Permission, Role};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

// 定义 RoleRepository trait，抽象角色数据的数据库操作
#[async_trait]
pub trait RoleRepository: Send + Sync {
    // 创建一个新角色
    async fn create(&self, name: &str, description: Option<&str>) -> Result<Role>;
    // 根据角色id查找角色
    async fn find_by_id(&self, role_id: Uuid) -> Result<Option<Role>>;
    // 根据角色名称查找角色
    async fn find_by_name(&self, name: &str) -> Result<Option<Role>>;
    // 给角色分配权限
    async fn assign_permissions_to_role(
        &self,
        role_id: Uuid,
        permission_ids: &[Uuid],
    ) -> Result<()>;
    // 获取某个角色的所有权限
    async fn get_permissions_for_role(&self, role_id: Uuid) -> Result<Vec<Permission>>;
    // 获取所有权限列表
    async fn list(&self) -> Result<Vec<Role>>;
    // 更新一个已存在的角色
    async fn update(&self, role_id: Uuid, payload: &UpdateRolePayload) -> Result<Role>;
    // 删除一个角色
    async fn delete(&self, role_id: Uuid) -> Result<()>;
}

#[derive(Clone)]
pub struct PostgresRoleRepository {
    pool: PgPool,
}

impl PostgresRoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for PostgresRoleRepository {
    async fn create(&self, name: &str, description: Option<&str>) -> Result<Role> {
        let role_id = Uuid::new_v4();

        let role = sqlx::query_as!(
            Role,
            r#"
            insert into roles (id, name, description) VALUES ($1,$2,$3)
            returning id,name,description,created_at,updated_at
            "#,
            role_id,
            name,
            description
        )
            .fetch_one(&self.pool)
            .await
            .context(format!("创建 角色 {} 失败", name))?;

        Ok(role)
    }

    async fn find_by_id(&self, role_id: Uuid) -> Result<Option<Role>> {
        let role = sqlx::query_as!(
            Role,
            r#"
            select id,name,description,created_at,updated_at from roles where id = $1
            "#,
            role_id
        )
            .fetch_optional(&self.pool)
            .await?;
        Ok(role)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Role>> {
        let role = sqlx::query_as!(
            Role,
            r#"
            select id,name,description,created_at,updated_at from roles 
            where name = $1
            "#,
            name
        )
            .fetch_optional(&self.pool)
            .await
            .context(format!("查找角色 {} 失败", name))?;
        Ok(role)
    }

    async fn assign_permissions_to_role(
        &self,
        role_id: Uuid,
        permission_ids: &[Uuid],
    ) -> Result<()> {
        // 如果权限 ID 列表为空，则无需执行任何操作
        if permission_ids.is_empty() {
            return Ok(());
        }
        // 使用 PostgreSQL 的 UNNEST 函数进行高效的批量插入
        // 语法解释:
        // - SELECT $1, permission_id FROM UNNEST($2::uuid[]) ...
        //   - UNNEST($2::uuid[]) 将传入的 UUID 数组参数 ($2) 展开成一个名为 permission_id 的临时列
        //   - SELECT $1, permission_id 从中选出 role_id (固定值 $1) 和每个展开的 permission_id
        // - INSERT INTO role_permissions ... SELECT ...
        //   - 将上述 SELECT 查询的结果集一次性插入到 role_permissions 表中
        // - ON CONFLICT DO NOTHING 忽略任何因主键冲突而导致的插入失败
        let _roles = sqlx::query!(
            r#"
            insert into role_permissions (role_id, permission_id) 
            select $1,permission_id from unnest($2::uuid[]) as t(permission_id)
            on conflict (role_id,permission_id) do nothing 
            "#,
            role_id,
            permission_ids
        )
            .execute(&self.pool)
            .await
            .context(format!("为用户id: {} 分配角色失败", role_id))?;

        Ok(())
    }

    async fn get_permissions_for_role(&self, role_id: Uuid) -> Result<Vec<Permission>> {
        let permissions = sqlx::query_as!(
            Permission,
            r#"
           select p.id,p.name,p.description,p.created_at,p.updated_at
           from permissions p
           inner join role_permissions rp on p.id =  rp.permission_id
           where rp.role_id = $1 order by p.name
           "#,
            role_id
        )
            .fetch_all(&self.pool)
            .await
            .context(format!("获取角色id {} 的所有权限失败", role_id))?;

        Ok(permissions)
    }

    async fn list(&self) -> Result<Vec<Role>> {
        sqlx::query_as!(
            Role,
            "select id,name,description,created_at,updated_at from roles order by name"
        )
            .fetch_all(&self.pool)
            .await
            .context("数据库层查询角色列表失败")
    }

    async fn update(&self, role_id: Uuid, payload: &UpdateRolePayload) -> Result<Role> {
        // 获取当前角色，以便处理部分更新
        let mut role = self.find_by_id(role_id).await?
            .ok_or_else(|| anyhow::anyhow!("未找到ID为 {} 的角色", role_id))?;

        if let Some(name) = &payload.name {
            role.name = name.clone()
        }
        if let Some(description) = &payload.description {
            role.description = Some(description.clone())
        }
        let updated_role = sqlx::query_as!(
            Role,
            r#"
            update roles
            set name = $1,description = $2,updated_at = now()
            where id = $3
            returning *
            "#,
            role.name,
            role.description,
            role_id
        )
            .fetch_one(&self.pool)
            .await?;
        Ok(updated_role)
    }

    async fn delete(&self, role_id: Uuid) -> Result<()> {
        let result = sqlx::query!(
            "delete from roles where id = $1",
            role_id
        )
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("尝试删除角色失败：未找到ID为 {} 的角色", role_id));
        }
        Ok(())
    }
}
