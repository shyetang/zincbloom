use crate::models::Category;
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    /// 创建一个新的分类
    /// name: 分类名称
    /// slug： 根据名称生成 URL slug
    async fn create(&self, name: &str, slug: &str) -> Result<Category>;

    /// 根据 ID 获取分类
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Category>>;

    /// 根据 slug 获取分类
    async fn get_by_slug(&self, slug: &str) -> Result<Option<Category>>;

    /// 获取所有分类列表
    async fn list(&self) -> Result<Vec<Category>>;

    /// 更新分类信息
    /// id: 要更新的分类的 ID
    /// name_opt: 可选的新名称。如果Some，则更新名称
    /// new_slug_opt: 可选的新 slug。如果Some，则更新slug
    /// 通常 slug 的更新与 name的更新联动，并在Service层处理
    /// Repository 层只负责执行数据库操作
    async fn update(
        &self,
        id: Uuid,
        name_opt: Option<&str>,
        new_slug_opt: Option<&str>,
    ) -> Result<Category>;

    /// 根据 ID 删除分类
    async fn delete(&self, id: Uuid) -> Result<()>;

    // 以后可能用到的方法
    // ///检查具有给定名称的分类是否存在
    // async fn name_exists(&self,name:&str)->Result<bool>;
    // /// 检查具有给定 slug 的分类是否已存在
    // async fn slug_exists(&self,slug:&str)->Result<bool>;
}

#[derive(Clone)]
pub struct PostgresCategoryRepository {
    pool: PgPool,
}

impl PostgresCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for PostgresCategoryRepository {
    async fn create(&self, name: &str, slug: &str) -> Result<Category> {
        let category_id = Uuid::new_v4();
        // created_at 和 updated_at 将使用数据库的 DEFAULT NOW()
        let category = sqlx::query_as!(
            Category,
            r#"
            insert into categories (id,name,slug)
            values ($1,$2,$3)
            returning id,name,slug,created_at,updated_at
            "#,
            category_id,
            name,
            slug
        )
        .fetch_one(&self.pool)
        .await
        .context("创建 Category 记录失败 (INSERT)")?;

        Ok(category)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Category>> {
        let category = sqlx::query_as!(
            Category,
            r#"
            select id,name,slug,created_at,updated_at
            from categories
            where id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context(format!("通过 ID ({}) 查询 Category 失败", id))?;

        Ok(category)
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<Category>> {
        let category = sqlx::query_as!(
            Category,
            r#"
            select id,name,slug,created_at,updated_at
            from categories
            where slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await
        .context(format!("通过Slug ({}) 查询 Category 失败", slug))?;

        Ok(category)
    }

    async fn list(&self) -> Result<Vec<Category>> {
        let categories = sqlx::query_as!(
            Category,
            r#"
            select id,name,slug,created_at,updated_at
            from categories
            order by name ASC  -- 按名称字幕顺序排序
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("查询 Category 列表失败")?;

        Ok(categories)
    }

    async fn update(
        &self,
        id: Uuid,
        name_opt: Option<&str>,
        new_slug_opt: Option<&str>,
    ) -> Result<Category> {
        // 1. 先获取当前分类数据，以便只更新提供的字段，或者基于当前值进行更新
        let mut current_category = self
            .get_by_id(id)
            .await
            .context(format!(
                "更新操作： 获取 Category (ID: {}) 失败以便更新",
                id
            ))?
            .ok_or_else(|| anyhow::anyhow!("更新目标 Category (ID: {}) 未找到", id))?;

        // 2.根据提供的参数更新字段
        if let Some(new_name) = name_opt {
            current_category.name = new_name.to_string();
        }
        if let Some(s) = new_slug_opt {
            current_category.slug = s.to_string();
        }
        // updated_at 将由数据库触发器自动更新，无需在此设置 current_category.updated_at
        // 3.执行SQL update
        let updated_category = sqlx::query_as!(
            Category,
            r#"
            update categories
            set name = $1,slug = $2
            where id = $3
            returning id,name,slug,created_at,updated_at
            "#,
            current_category.name,
            current_category.slug,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context(format!("数据库层面更新 Category (ID: {}) 失败", id))?;

        Ok(updated_category)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query!("delete from categories where id = $1", id)
            .execute(&self.pool)
            .await
            .context(format!("删除 Category (ID: {})", id))?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!(
                "尝试删除 Category (ID: {}) 时未找到记录",
                id
            ));
        }

        Ok(())
    }
}
