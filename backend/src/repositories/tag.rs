use crate::models::Tag;
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

// 定义 TagRepository trait,用于Tag的数据库操作
#[async_trait]
pub trait TagRepository: Send + Sync {
    // Send 和 Sync trait 使得 TagRepository 可以在多线程环境下安全共享
    /// 创建一个新的标签。
    ///
    /// # 参数
    ///
    /// * `name` - 标签的名称。
    /// * `slug` - 标签的 URL友好型 slug。
    async fn create(&self, name: &str, slug: &str) -> Result<Tag>;

    /// 根据 UUID 检索标签
    ///
    /// # 参数
    ///
    /// * `id` - 标签的 UUID。
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Tag>>; // 返回 Option<Tag> 因为标签可能不存在

    /// 根据 slug 检索标签
    ///
    /// # 参数
    ///
    /// * `slug` - 标签的 slug。
    async fn get_by_slug(&self, slug: &str) -> Result<Option<Tag>>;

    /// 列出所有标签
    async fn list(&self) -> Result<Vec<Tag>>;

    /// 更新一个已存在的标签
    ///
    /// # 参数
    ///
    /// * `id` - 需要更新的标签的 UUID
    /// * `name_opt` - 可选的新标签名称。
    /// * `new_slug_opt` - 可选的新标签 slug。
    async fn update(
        &self,
        id: Uuid,
        name_opt: Option<&str>,
        new_slug_opt: Option<&str>,
    ) -> Result<Tag>;

    /// 根据 UUID 删除标签
    ///
    /// # 参数
    ///
    /// * `id` - 需要删除的标签的 UUID
    async fn delete(&self, id: Uuid) -> Result<()>; // 删除成功则返回空元组 ()

    // 以后可能添加的方法
    // async fn name_exists(&self, name: &str) -> Result<bool>; // 检查名称是否存在
    // async fn slug_exists(&self, slug: &str) -> Result<bool>; // 检查slug是否存在
}

// TagRepository 的 Postgres 具体实现
#[derive(Clone)]
pub struct PostgresTagRepository {
    pool: PgPool, // 数据库连接池
}

impl PostgresTagRepository {
    // 构造函数
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagRepository for PostgresTagRepository {
    async fn create(&self, name: &str, slug: &str) -> Result<Tag> {
        let tag_id = Uuid::new_v4(); // 为新标签生成 UUID

        // `created_at` 和 `updated_at` 会使用数据库定义的 DEFAULT NOW()
        let tag = sqlx::query_as!(
            // sqlx::query_as! 宏在编译时检查 SQL 查询并映射结果到 Tag 结构体
            Tag, // 目标结构体
            r#"    -- SQL查询语句，returning 子句返回插入的行
            insert into tags (id,name,slug)
            values ($1,$2,$3)
            returning id,name,slug,created_at,updated_at
            "#,
            tag_id,
            name,
            slug
        )
        .fetch_one(&self.pool) // 执行查询并获取一行结果
        .await // 等待异步操作完成
        .context(format!("创建标签 '{}' 失败", name))?; // 如果出错，添加上下文信息

        Ok(tag) // 返回创建的标签
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Tag>> {
        let tag = sqlx::query_as!(
            Tag,
            r#"
            select id,name,slug,created_at,updated_at
            from tags
            where id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool) // fetch_optional 获取零行或一行结果
        .await
        .context(format!("通过 ID ('{}') 查询标签失败", id))?;

        Ok(tag)
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<Tag>> {
        let tag = sqlx::query_as!(
            Tag,
            r#"
            select id,name,slug,created_at,updated_at
            from tags
            where slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool) // fetch_optional 获取零行或一行结果
        .await
        .context(format!("通过 Slug ('{}') 查询标签失败", slug))?;

        Ok(tag)
    }

    async fn list(&self) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as!(
            Tag,
            r#"
            select id,name,slug,created_at,updated_at
            from tags
            order by name ASC  -- 按名称升序排序
            "#
        )
        .fetch_all(&self.pool) // fetch_all 获取所有结果行
        .await
        .context("查询标签列表失败")?;

        Ok(tags)
    }

    async fn update(
        &self,
        id: Uuid,
        name_opt: Option<&str>,     // 可选的新名称
        new_slug_opt: Option<&str>, // 可选的新 slug
    ) -> Result<Tag> {
        // 1. 先获取当前标签数据，以便只更新提供的字段
        let mut current_tag = self
            .get_by_id(id)
            .await
            .context(format!("更新操作：获取标签 (ID: {}) 失败", id))?
            .ok_or_else(|| anyhow!("更新目标标签 (ID: {}) 未找到", id))?; // 如果未找到，返回错误

        // 2. 根据提供的参数更新字段
        if let Some(new_name) = name_opt {
            current_tag.name = new_name.to_string();
        }
        if let Some(new_slug) = new_slug_opt {
            current_tag.slug = new_slug.to_string();
        }
        // 3. 执行SQL
        let updated_tag = sqlx::query_as!(
            Tag,
            r#"
            update tags
            set name = $1,slug = $2 -- updated_at 会自动更新
            where id = $3
            returning id,name,slug,created_at,updated_at
            "#,
            current_tag.name,
            current_tag.slug,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context(format!("数据库层面更新标签 (ID: {}) 失败", id))?;

        Ok(updated_tag)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query!(
            r#"
            delete from tags
            where id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .context(format!("删除标签 (ID: {}) 失败", id))?;

        // 检查是否有行受到影响
        if result.rows_affected() == 0 {
            // 如果没有行被删除，意味着具有该 ID 的标签不存在
            Err(anyhow!("尝试删除标签 (ID: {}) 时未找到记录", id))
        } else {
            Ok(()) // 删除成功
        }
    }
}
