use crate::models::Tag;
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

/// 标签使用统计
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TagUsageStats {
    pub tag: Tag,
    pub post_count: i64,
}

/// 相似标签组（用于管理员清理重复标签）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimilarTagGroup {
    pub tags: Vec<Tag>,
    pub similarity_reason: String, // 例如："名称相似"、"同义词"等
}

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

    // === 管理员专用功能 ===

    /// 合并标签：将source_tag_ids中的所有标签的文章关联转移到target_tag_id，然后删除源标签
    async fn merge_tags(&self, target_tag_id: Uuid, source_tag_ids: &[Uuid]) -> Result<()>;

    /// 增强版标签合并：返回详细的操作结果
    async fn merge_tags_enhanced(
        &self,
        target_tag_id: Uuid,
        source_tag_ids: &[Uuid],
        new_target_name: Option<&str>,
    ) -> Result<(usize, usize, usize)>;

    /// 获取标签合并预览信息
    async fn get_merge_preview(
        &self,
        target_tag_id: Uuid,
        source_tag_ids: &[Uuid],
    ) -> Result<(Vec<crate::models::Tag>, usize, usize)>;

    /// 批量删除标签
    async fn batch_delete(&self, tag_ids: &[Uuid]) -> Result<usize>;

    /// 增强版批量删除：支持孤儿文章处理
    async fn batch_delete_enhanced(
        &self,
        tag_ids: &[Uuid],
        handle_orphaned: bool,
    ) -> Result<(usize, usize, usize)>;

    /// 获取标签使用统计
    async fn get_usage_stats(&self) -> Result<Vec<TagUsageStats>>;

    /// 查找相似标签
    async fn find_similar_tags(&self) -> Result<Vec<SimilarTagGroup>>;

    /// 获取单个标签的文章关联信息
    async fn get_tag_post_info(&self, tag_id: Uuid) -> Result<(usize, Vec<String>)>;

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

    // === 管理员专用功能的实现 ===

    async fn merge_tags(&self, target_tag_id: Uuid, source_tag_ids: &[Uuid]) -> Result<()> {
        let mut txn = self.pool.begin().await.context("开始标签合并事务失败")?;

        // 1. 将所有源标签的文章关联转移到目标标签
        for &source_id in source_tag_ids {
            // 更新post_tags表，将source_id替换为target_tag_id
            sqlx::query!(
                r#"
                UPDATE post_tags 
                SET tag_id = $1 
                WHERE tag_id = $2 
                AND NOT EXISTS (
                    SELECT 1 FROM post_tags 
                    WHERE post_id = post_tags.post_id AND tag_id = $1
                )
                "#,
                target_tag_id,
                source_id
            )
            .execute(&mut *txn)
            .await
            .context(format!("转移标签 {} 的文章关联失败", source_id))?;

            // 删除重复的关联（如果某篇文章同时有源标签和目标标签）
            sqlx::query!("DELETE FROM post_tags WHERE tag_id = $1", source_id)
                .execute(&mut *txn)
                .await
                .context(format!("删除标签 {} 的关联失败", source_id))?;
        }

        // 2. 删除源标签
        for &source_id in source_tag_ids {
            sqlx::query!("DELETE FROM tags WHERE id = $1", source_id)
                .execute(&mut *txn)
                .await
                .context(format!("删除源标签 {} 失败", source_id))?;
        }

        txn.commit().await.context("提交标签合并事务失败")?;
        Ok(())
    }

    async fn merge_tags_enhanced(
        &self,
        target_tag_id: Uuid,
        source_tag_ids: &[Uuid],
        new_target_name: Option<&str>,
    ) -> Result<(usize, usize, usize)> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("开始增强标签合并事务失败")?;

        let mut total_affected_posts = 0;
        let mut duplicate_relations_removed = 0;

        // 1. 统计受影响的文章数和重复关联数
        for &source_id in source_tag_ids {
            // 计算有多少文章将受影响
            let affected_posts = sqlx::query!(
                "SELECT COUNT(DISTINCT post_id) as count FROM post_tags WHERE tag_id = $1",
                source_id
            )
            .fetch_one(&mut *txn)
            .await?;
            total_affected_posts += affected_posts.count.unwrap_or(0) as usize;

            // 计算重复关联数
            let duplicates = sqlx::query!(
                r#"
                SELECT COUNT(*) as count FROM post_tags pt1
                WHERE pt1.tag_id = $1 
                AND EXISTS (
                    SELECT 1 FROM post_tags pt2 
                    WHERE pt2.post_id = pt1.post_id AND pt2.tag_id = $2
                )
                "#,
                source_id,
                target_tag_id
            )
            .fetch_one(&mut *txn)
            .await?;
            duplicate_relations_removed += duplicates.count.unwrap_or(0) as usize;
        }

        // 2. 执行合并操作
        for &source_id in source_tag_ids {
            // 转移非重复的关联
            sqlx::query!(
                r#"
                UPDATE post_tags 
                SET tag_id = $1 
                WHERE tag_id = $2 
                AND NOT EXISTS (
                    SELECT 1 FROM post_tags 
                    WHERE post_id = post_tags.post_id AND tag_id = $1
                )
                "#,
                target_tag_id,
                source_id
            )
            .execute(&mut *txn)
            .await?;

            // 删除所有源标签关联
            sqlx::query!("DELETE FROM post_tags WHERE tag_id = $1", source_id)
                .execute(&mut *txn)
                .await?;
        }

        // 3. 更新目标标签名称（如果提供）
        if let Some(new_name) = new_target_name {
            let new_slug = slug::slugify(new_name);
            sqlx::query!(
                "UPDATE tags SET name = $1, slug = $2 WHERE id = $3",
                new_name,
                new_slug,
                target_tag_id
            )
            .execute(&mut *txn)
            .await?;
        }

        // 4. 删除源标签
        for &source_id in source_tag_ids {
            sqlx::query!("DELETE FROM tags WHERE id = $1", source_id)
                .execute(&mut *txn)
                .await?;
        }

        txn.commit().await.context("提交增强标签合并事务失败")?;

        // 返回 (受影响的文章数, 合并的标签数, 移除的重复关联数)
        Ok((
            total_affected_posts,
            source_tag_ids.len(),
            duplicate_relations_removed,
        ))
    }

    async fn get_merge_preview(
        &self,
        target_tag_id: Uuid,
        source_tag_ids: &[Uuid],
    ) -> Result<(Vec<crate::models::Tag>, usize, usize)> {
        let mut all_tag_ids = vec![target_tag_id];
        all_tag_ids.extend_from_slice(source_tag_ids);

        // 获取所有相关标签
        let mut tags = Vec::new();
        for &tag_id in &all_tag_ids {
            if let Some(tag) = self.get_by_id(tag_id).await? {
                tags.push(tag);
            }
        }

        // 计算总的受影响文章数
        let total_posts_query = format!(
            "SELECT COUNT(DISTINCT post_id) as count FROM post_tags WHERE tag_id = ANY($1::uuid[])"
        );
        let total_posts = sqlx::query_scalar::<_, i64>(&total_posts_query)
            .bind(&all_tag_ids)
            .fetch_one(&self.pool)
            .await?;

        // 计算重复关联数
        let mut duplicate_count = 0;
        for &source_id in source_tag_ids {
            let duplicates = sqlx::query!(
                r#"
                SELECT COUNT(*) as count FROM post_tags pt1
                WHERE pt1.tag_id = $1 
                AND EXISTS (
                    SELECT 1 FROM post_tags pt2 
                    WHERE pt2.post_id = pt1.post_id AND pt2.tag_id = $2
                )
                "#,
                source_id,
                target_tag_id
            )
            .fetch_one(&self.pool)
            .await?;
            duplicate_count += duplicates.count.unwrap_or(0) as usize;
        }

        Ok((tags, total_posts as usize, duplicate_count))
    }

    async fn batch_delete(&self, tag_ids: &[Uuid]) -> Result<usize> {
        if tag_ids.is_empty() {
            return Ok(0);
        }

        let mut deleted_count = 0;
        for &tag_id in tag_ids {
            let result = sqlx::query!("DELETE FROM tags WHERE id = $1", tag_id)
                .execute(&self.pool)
                .await
                .context(format!("批量删除：删除标签 {} 失败", tag_id))?;

            deleted_count += result.rows_affected() as usize;
        }

        Ok(deleted_count)
    }

    async fn batch_delete_enhanced(
        &self,
        tag_ids: &[Uuid],
        handle_orphaned: bool,
    ) -> Result<(usize, usize, usize)> {
        if tag_ids.is_empty() {
            return Ok((0, 0, 0));
        }

        let mut txn = self
            .pool
            .begin()
            .await
            .context("开始增强批量删除事务失败")?;

        // 1. 统计受影响的文章数
        let affected_posts_query = format!(
            "SELECT COUNT(DISTINCT post_id) as count FROM post_tags WHERE tag_id = ANY($1::uuid[])"
        );
        let affected_posts = sqlx::query_scalar::<_, i64>(&affected_posts_query)
            .bind(tag_ids)
            .fetch_one(&mut *txn)
            .await?;

        // 2. 删除标签关联
        for &tag_id in tag_ids {
            sqlx::query!("DELETE FROM post_tags WHERE tag_id = $1", tag_id)
                .execute(&mut *txn)
                .await?;
        }

        // 3. 统计孤儿文章数（没有任何标签的文章）
        let orphaned_posts = if handle_orphaned {
            let orphaned_count = sqlx::query!(
                r#"
                SELECT COUNT(*) as count FROM posts p
                WHERE NOT EXISTS (SELECT 1 FROM post_tags pt WHERE pt.post_id = p.id)
                "#
            )
            .fetch_one(&mut *txn)
            .await?;
            orphaned_count.count.unwrap_or(0) as usize
        } else {
            0
        };

        // 4. 删除标签
        let mut deleted_count = 0;
        for &tag_id in tag_ids {
            let result = sqlx::query!("DELETE FROM tags WHERE id = $1", tag_id)
                .execute(&mut *txn)
                .await?;
            deleted_count += result.rows_affected() as usize;
        }

        txn.commit().await.context("提交增强批量删除事务失败")?;

        // 返回 (删除的标签数, 受影响的文章数, 孤儿文章数)
        Ok((deleted_count, affected_posts as usize, orphaned_posts))
    }

    async fn get_usage_stats(&self) -> Result<Vec<TagUsageStats>> {
        let stats = sqlx::query!(
            r#"
            SELECT 
                t.id, t.name, t.slug, t.created_at, t.updated_at,
                COALESCE(COUNT(pt.post_id), 0) as post_count
            FROM tags t
            LEFT JOIN post_tags pt ON t.id = pt.tag_id
            GROUP BY t.id, t.name, t.slug, t.created_at, t.updated_at
            ORDER BY post_count DESC, t.name ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("获取标签使用统计失败")?;

        let mut usage_stats = Vec::new();
        for row in stats {
            let tag = Tag {
                id: row.id,
                name: row.name,
                slug: row.slug,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            usage_stats.push(TagUsageStats {
                tag,
                post_count: row.post_count.unwrap_or(0),
            });
        }

        Ok(usage_stats)
    }

    async fn find_similar_tags(&self) -> Result<Vec<SimilarTagGroup>> {
        // 这是一个简化的实现，查找名称相似的标签
        let tags = self.list().await?;
        let mut similar_groups = Vec::new();

        // 查找名称长度相似且编辑距离小的标签
        for i in 0..tags.len() {
            for j in (i + 1)..tags.len() {
                let tag1 = &tags[i];
                let tag2 = &tags[j];

                // 简单的相似性检查：名称长度相近且包含相似字符
                if is_similar(&tag1.name, &tag2.name) {
                    similar_groups.push(SimilarTagGroup {
                        tags: vec![tag1.clone(), tag2.clone()],
                        similarity_reason: "名称相似".to_string(),
                    });
                }
            }
        }

        Ok(similar_groups)
    }

    async fn get_tag_post_info(&self, tag_id: Uuid) -> Result<(usize, Vec<String>)> {
        // 获取文章数量
        let post_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM post_tags WHERE tag_id = $1",
            tag_id
        )
        .fetch_one(&self.pool)
        .await?;

        // 获取示例文章标题（最多5个）
        let sample_titles = sqlx::query!(
            r#"
            SELECT p.title 
            FROM posts p 
            JOIN post_tags pt ON p.id = pt.post_id 
            WHERE pt.tag_id = $1 
            ORDER BY p.created_at DESC 
            LIMIT 5
            "#,
            tag_id
        )
        .fetch_all(&self.pool)
        .await?;

        let titles: Vec<String> = sample_titles.into_iter().map(|row| row.title).collect();

        Ok((post_count.count.unwrap_or(0) as usize, titles))
    }
}

// 辅助函数：简单的相似性检查
fn is_similar(name1: &str, name2: &str) -> bool {
    let name1_lower = name1.to_lowercase();
    let name2_lower = name2.to_lowercase();

    // 检查是否一个包含另一个，或者长度相近且有共同子串
    if name1_lower.contains(&name2_lower) || name2_lower.contains(&name1_lower) {
        return true;
    }

    // 检查是否有共同的长子串（简化版本）
    let min_len = name1_lower.len().min(name2_lower.len());
    if min_len >= 3 {
        for i in 0..=(name1_lower.len() - 3) {
            let substr = &name1_lower[i..i + 3];
            if name2_lower.contains(substr) {
                return true;
            }
        }
    }

    false
}
