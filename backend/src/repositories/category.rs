use crate::models::Category;
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

/// 分类使用统计
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryUsageStats {
    pub category: Category,
    pub post_count: i64,
}

/// 相似分类组（用于管理员清理重复分类）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimilarCategoryGroup {
    pub categories: Vec<Category>,
    pub similarity_reason: String, // 例如："名称相似"、"同义词"等
}

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

    // === 管理员专用功能 ===

    /// 合并分类：将source_category_ids中的所有分类的文章关联转移到target_category_id，然后删除源分类
    async fn merge_categories(
        &self,
        target_category_id: Uuid,
        source_category_ids: &[Uuid],
    ) -> Result<()>;

    /// 增强版分类合并：返回详细的操作结果
    async fn merge_categories_enhanced(
        &self,
        target_category_id: Uuid,
        source_category_ids: &[Uuid],
        new_target_name: Option<&str>,
    ) -> Result<(usize, usize, usize)>;

    /// 获取分类合并预览信息
    async fn get_merge_preview(
        &self,
        target_category_id: Uuid,
        source_category_ids: &[Uuid],
    ) -> Result<(Vec<Category>, usize, usize)>;

    /// 批量删除分类
    async fn batch_delete(&self, category_ids: &[Uuid]) -> Result<usize>;

    /// 增强版批量删除：支持孤儿文章处理
    async fn batch_delete_enhanced(
        &self,
        category_ids: &[Uuid],
        handle_orphaned: bool,
    ) -> Result<(usize, usize, usize)>;

    /// 获取分类使用统计
    async fn get_usage_stats(&self) -> Result<Vec<CategoryUsageStats>>;

    /// 查找相似分类
    async fn find_similar_categories(&self) -> Result<Vec<SimilarCategoryGroup>>;

    /// 获取单个分类的文章关联信息
    async fn get_category_post_info(&self, category_id: Uuid) -> Result<(usize, Vec<String>)>;

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
            order by name  -- 按名称字幕顺序排序
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

    // === 管理员专用功能实现 ===

    async fn merge_categories(
        &self,
        target_category_id: Uuid,
        source_category_ids: &[Uuid],
    ) -> Result<()> {
        let mut txn = self.pool.begin().await.context("开始事务失败")?;

        for &source_id in source_category_ids {
            // 转移文章关联（避免重复）
            sqlx::query!(
                r#"
                UPDATE post_categories
                SET category_id = $1
                WHERE category_id = $2
                AND NOT EXISTS (
                    SELECT 1 FROM post_categories pc2
                    WHERE pc2.post_id = post_categories.post_id AND pc2.category_id = $1
                )
                "#,
                target_category_id,
                source_id
            )
            .execute(&mut *txn)
            .await
            .context("转移分类关联失败")?;

            // 删除剩余的源分类关联
            sqlx::query!(
                "DELETE FROM post_categories WHERE category_id = $1",
                source_id
            )
            .execute(&mut *txn)
            .await
            .context("删除剩余关联失败")?;

            // 删除源分类
            sqlx::query!("DELETE FROM categories WHERE id = $1", source_id)
                .execute(&mut *txn)
                .await
                .context("删除源分类失败")?;
        }

        txn.commit().await.context("提交分类合并事务失败")?;
        Ok(())
    }

    async fn merge_categories_enhanced(
        &self,
        target_category_id: Uuid,
        source_category_ids: &[Uuid],
        new_target_name: Option<&str>,
    ) -> Result<(usize, usize, usize)> {
        let mut txn = self.pool.begin().await.context("开始增强合并事务失败")?;

        let mut total_affected_posts = 0;
        let mut total_duplicates_removed = 0;
        let merged_category_count = source_category_ids.len();

        for &source_id in source_category_ids {
            // 获取受影响的文章数
            let affected_posts = sqlx::query_scalar!(
                r#"
                SELECT COUNT(*)::int as count
                FROM post_categories
                WHERE category_id = $1
                "#,
                source_id
            )
            .fetch_one(&mut *txn)
            .await
            .context("统计受影响文章数失败")?
            .unwrap_or(0) as usize;

            total_affected_posts += affected_posts;

            // 转移文章关联（避免重复）
            let transferred_rows = sqlx::query!(
                r#"
                UPDATE post_categories
                SET category_id = $1
                WHERE category_id = $2
                AND NOT EXISTS (
                    SELECT 1 FROM post_categories pc2
                    WHERE pc2.post_id = post_categories.post_id AND pc2.category_id = $1
                )
                "#,
                target_category_id,
                source_id
            )
            .execute(&mut *txn)
            .await
            .context("转移分类关联失败")?;

            // 删除剩余的源分类关联（转移后应该只剩重复的关联）
            let deleted_rows = sqlx::query!(
                "DELETE FROM post_categories WHERE category_id = $1",
                source_id
            )
            .execute(&mut *txn)
            .await
            .context("删除剩余关联失败")?;

            // 记录删除的重复关联数
            total_duplicates_removed += deleted_rows.rows_affected() as usize;

            // 验证数据完整性：转移数 + 删除数应该等于原始数
            let expected_total = transferred_rows.rows_affected() + deleted_rows.rows_affected();
            if expected_total != affected_posts as u64 {
                return Err(anyhow::anyhow!(
                    "数据完整性检查失败：源分类 {} 预期影响 {} 篇文章，实际处理 {} 篇",
                    source_id,
                    affected_posts,
                    expected_total
                ));
            }

            // 删除源分类
            sqlx::query!("DELETE FROM categories WHERE id = $1", source_id)
                .execute(&mut *txn)
                .await
                .context("删除源分类失败")?;
        }

        // 如果提供了新名称，更新目标分类
        if let Some(new_name) = new_target_name {
            let new_slug = slug::slugify(new_name);
            sqlx::query!(
                "UPDATE categories SET name = $1, slug = $2 WHERE id = $3",
                new_name,
                new_slug,
                target_category_id
            )
            .execute(&mut *txn)
            .await
            .context("更新目标分类名称失败")?;
        }

        txn.commit().await.context("提交增强合并事务失败")?;

        Ok((
            merged_category_count,
            total_affected_posts,
            total_duplicates_removed,
        ))
    }

    async fn get_merge_preview(
        &self,
        target_category_id: Uuid,
        source_category_ids: &[Uuid],
    ) -> Result<(Vec<Category>, usize, usize)> {
        // 获取所有相关分类
        let mut all_category_ids = vec![target_category_id];
        all_category_ids.extend_from_slice(source_category_ids);

        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, slug, created_at, updated_at
            FROM categories
            WHERE id = ANY($1)
            "#,
            &all_category_ids
        )
        .fetch_all(&self.pool)
        .await
        .context("查询相关分类失败")?;

        // 统计总受影响文章数
        let total_affected = sqlx::query_scalar!(
            r#"
            SELECT COUNT(DISTINCT post_id)::int as count
            FROM post_categories
            WHERE category_id = ANY($1)
            "#,
            source_category_ids
        )
        .fetch_one(&self.pool)
        .await
        .context("统计受影响文章数失败")?
        .unwrap_or(0) as usize;

        // 统计重复关联数
        let duplicates = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)::int as count
            FROM post_categories pc1
            WHERE pc1.category_id = ANY($1)
            AND EXISTS (
                SELECT 1 FROM post_categories pc2
                WHERE pc2.post_id = pc1.post_id AND pc2.category_id = $2
            )
            "#,
            source_category_ids,
            target_category_id
        )
        .fetch_one(&self.pool)
        .await
        .context("统计重复关联数失败")?
        .unwrap_or(0) as usize;

        Ok((categories, total_affected, duplicates))
    }

    async fn batch_delete(&self, category_ids: &[Uuid]) -> Result<usize> {
        let result = sqlx::query!("DELETE FROM categories WHERE id = ANY($1)", category_ids)
            .execute(&self.pool)
            .await
            .context("批量删除分类失败")?;

        Ok(result.rows_affected() as usize)
    }

    async fn batch_delete_enhanced(
        &self,
        category_ids: &[Uuid],
        handle_orphaned: bool,
    ) -> Result<(usize, usize, usize)> {
        let mut txn = self.pool.begin().await.context("开始批量删除事务失败")?;

        // 统计受影响的文章数
        let affected_posts = sqlx::query_scalar!(
            r#"
            SELECT COUNT(DISTINCT post_id)::int as count
            FROM post_categories
            WHERE category_id = ANY($1)
            "#,
            category_ids
        )
        .fetch_one(&mut *txn)
        .await
        .context("统计受影响文章数失败")?
        .unwrap_or(0) as usize;

        // 如果需要处理孤儿文章
        let mut orphaned_posts = 0;
        if handle_orphaned {
            // 这里可以实现孤儿文章处理逻辑
            // 例如：给没有分类的文章添加"未分类"分类
        }

        // 删除分类关联
        sqlx::query!(
            "DELETE FROM post_categories WHERE category_id = ANY($1)",
            category_ids
        )
        .execute(&mut *txn)
        .await
        .context("删除分类关联失败")?;

        // 删除分类
        let result = sqlx::query!("DELETE FROM categories WHERE id = ANY($1)", category_ids)
            .execute(&mut *txn)
            .await
            .context("删除分类失败")?;

        let deleted_count = result.rows_affected() as usize;

        txn.commit().await.context("提交批量删除事务失败")?;

        Ok((deleted_count, affected_posts, orphaned_posts))
    }

    async fn get_usage_stats(&self) -> Result<Vec<CategoryUsageStats>> {
        let stats = sqlx::query!(
            r#"
            SELECT 
                c.id, c.name, c.slug, c.created_at, c.updated_at,
                COUNT(pc.post_id)::int8 as post_count
            FROM categories c
            LEFT JOIN post_categories pc ON c.id = pc.category_id
            GROUP BY c.id, c.name, c.slug, c.created_at, c.updated_at
            ORDER BY post_count DESC, c.name ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("获取分类使用统计失败")?;

        let usage_stats = stats
            .into_iter()
            .map(|row| CategoryUsageStats {
                category: Category {
                    id: row.id,
                    name: row.name,
                    slug: row.slug,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                },
                post_count: row.post_count.unwrap_or(0),
            })
            .collect();

        Ok(usage_stats)
    }

    async fn find_similar_categories(&self) -> Result<Vec<SimilarCategoryGroup>> {
        let categories = self.list().await?;
        let mut similar_groups = Vec::new();

        // 简单的相似性检测：名称相似度检查
        for i in 0..categories.len() {
            for j in (i + 1)..categories.len() {
                if is_similar(&categories[i].name, &categories[j].name) {
                    similar_groups.push(SimilarCategoryGroup {
                        categories: vec![categories[i].clone(), categories[j].clone()],
                        similarity_reason: "名称相似".to_string(),
                    });
                }
            }
        }

        Ok(similar_groups)
    }

    async fn get_category_post_info(&self, category_id: Uuid) -> Result<(usize, Vec<String>)> {
        let post_count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)::int as count
            FROM post_categories
            WHERE category_id = $1
            "#,
            category_id
        )
        .fetch_one(&self.pool)
        .await
        .context("获取分类文章数失败")?
        .unwrap_or(0) as usize;

        let sample_titles: Vec<String> = sqlx::query_scalar!(
            r#"
            SELECT p.title
            FROM posts p
            JOIN post_categories pc ON p.id = pc.post_id
            WHERE pc.category_id = $1
            ORDER BY p.created_at DESC
            LIMIT 5
            "#,
            category_id
        )
        .fetch_all(&self.pool)
        .await
        .context("获取示例文章标题失败")?;

        Ok((post_count, sample_titles))
    }
}

// 检查两个分类名称是否相似
fn is_similar(name1: &str, name2: &str) -> bool {
    let name1_lower = name1.to_lowercase();
    let name2_lower = name2.to_lowercase();

    // 检查是否一个是另一个的子字符串
    if name1_lower.contains(&name2_lower) || name2_lower.contains(&name1_lower) {
        return true;
    }

    // 检查编辑距离
    let distance = levenshtein_distance(&name1_lower, &name2_lower);
    let max_len = name1_lower.len().max(name2_lower.len());

    // 如果编辑距离小于最大长度的30%，认为相似
    if max_len > 0 {
        let similarity_threshold = 0.3;
        (distance as f64) / (max_len as f64) < similarity_threshold
    } else {
        false
    }
}

// 计算编辑距离
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[a_len][b_len]
}
