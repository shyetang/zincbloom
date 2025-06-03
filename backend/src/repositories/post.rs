use crate::dtos::post::{CategoryDto, CreatePostPayload, TagDto, UpdatePostPayload};
use crate::models::Post;
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

// 定义仓库操作的 trait
#[async_trait]
pub trait PostRepository: Send + Sync {
    // Send + Sync 是因为要在async Axum中共享
    async fn create(&self, payload: &CreatePostPayload, slug: &str) -> Result<Post>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Post>>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<Post>>;
    async fn list(&self, limit: i64, offset: i64) -> Result<(Vec<Post>, i64)>;

    async fn update(
        &self,
        id: Uuid,
        payload: &UpdatePostPayload,
        new_slug: Option<&str>,
    ) -> Result<Post>;

    async fn delete(&self, id: Uuid) -> Result<()>;

    // 获取帖子的完整分类和标签对象
    async fn get_categories_for_post(&self, post_id: Uuid) -> Result<Vec<CategoryDto>>;
    async fn get_tags_for_post(&self, post_id: Uuid) -> Result<Vec<TagDto>>;
    // 获取帖子的分类和标签IDs
    // 暂时不实现，但作为未来获取关联信息的占位
    // async fn get_category_ids_for_post(&self, post_id: Uuid, pool: &PgPool) -> Result<Vec<Uuid>>;
    // async fn get_tag_ids_for_post(&self, post_id: Uuid, pool: &PgPool) -> Result<Vec<Uuid>>;
}

// Postgres的具体实现
#[derive(Clone)] // Clone是为了能在 Axum state 中共享
pub struct PostgresPostRepository {
    pool: PgPool,
}

impl PostgresPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // 辅助函数：在事务中管理帖子的分类关联
    async fn manage_post_categories(
        txn: &mut Transaction<'_, Postgres>,
        post_id: Uuid,
        category_ids: &Option<Vec<Uuid>>,
        is_update: bool, // 用来区分是创建操作还是更新操作。如果是更新，会先删除该帖子的所有旧关联。
    ) -> Result<()> {
        if is_update {
            // 更新时，先删除旧的关联
            sqlx::query!("delete from post_categories where post_id = $1", post_id)
                .execute(&mut **txn) // 使用 txn
                .await
                .context("Failed to delete old post categories")?;
        }
        if let Some(ids) = category_ids {
            if !ids.is_empty() {
                // 准备批量插入
                // SQLx 的 query! 和 query_as! 不直接支持 VALUES ($1, $2), ($1, $3), ... 这样的动态多行插入
                // 我们需要循环插入，或者构建更复杂的查询字符串 (后者不推荐，易出错且不安全)
                // 或者使用 sqlx::QueryBuilder (更安全地构建动态查询)
                // 为了简单起见，这里先用循环。对于大量ID，性能可能不是最优，但对于少量ID是可接受的。
                for category_id in ids {
                    sqlx::query!(
                        "insert into post_categories (post_id, category_id) values ($1,$2) on conflict do nothing",
                        post_id,
                        category_id
                    )
                        .execute(&mut **txn)
                        .await
                        .context(format!("Failed to associate post {} with category {}",post_id,category_id))?;
                }
            }
        }
        Ok(())
    }

    // 辅助函数：在事务中管理帖子的标签关联
    async fn manage_post_tags(
        txn: &mut Transaction<'_, Postgres>,
        post_id: Uuid,
        tag_ids: &Option<Vec<Uuid>>,
        is_update: bool,
    ) -> Result<()> {
        if is_update {
            sqlx::query!("delete from post_tags where post_id = $1", post_id)
                .execute(&mut **txn)
                .await
                .context("Failed to delete old tags")?;
        }
        if let Some(ids) = tag_ids {
            if !ids.is_empty() {
                for tag_id in ids {
                    sqlx::query!(
                        "insert into post_tags (post_id, tag_id) values ($1,$2) on conflict do nothing",
                        post_id,
                        tag_id
                    )
                        .execute(&mut **txn)
                        .await
                        .context(format!("Failed to associate post {} with tag {}",post_id,tag_id))?;
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn create(&self, payload: &CreatePostPayload, slug: &str) -> Result<Post> {
        let post_id = Uuid::new_v4();
        let now = Utc::now();

        // 开始数据库事务
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to begin transaction for creating post")?;

        // 1. 插入帖子基本信息
        let post = sqlx::query_as!(
            Post,
            r#"
            insert into posts (id,slug,title,content,created_at,updated_at,published_at)
            values ($1,$2,$3,$4,$5,$6,$7)
            returning id,slug,title,content,created_at,updated_at,published_at
            "#,
            post_id,
            slug,
            &payload.title,
            &payload.content,
            now,
            now,
            None::<DateTime<Utc>>
        )
        .fetch_one(&mut *txn) // 在事务中执行
        .await
        .context("未能将帖子基本信息插入到posts表中")?; // 使用 ？ 和 context

        // 2. 关联分类(如果提供了 category_ids)
        // is_update is false for creation
        Self::manage_post_categories(&mut txn, post.id, &payload.category_ids, false)
            .await
            .context("Failed to manage post categories during creation")?;

        // 3. 关联标签(如果提供了 tag_ids)
        // is_update is false for creation
        Self::manage_post_tags(&mut txn, post.id, &payload.tag_ids, false)
            .await
            .context("Failed to manage post tags during creation")?;

        // 提交事务
        txn.commit()
            .await
            .context("Failed to commit transaction for creating post")?;

        Ok(post)
    }

    // select id,slug,title,content, created_at as "created_at!",updated_at as "updated_at!",published_at from posts where id = $1
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Post>> {
        let post = sqlx::query_as!(
            Post,
            r#"
            select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at from posts where id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .context(format!("按 id ({}) 查询 Post 失败", id))?;
        Ok(post)
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<Post>> {
        let post = sqlx::query_as!(
            Post,
            r#"
            select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at 
            from posts 
            where slug = $1
            "#,
            slug
        )
            .fetch_optional(&self.pool)
            .await
            .context(format!("按 slug ({}) 查询 Post 失败", slug))?;
        Ok(post)
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<(Vec<Post>, i64)> {
        // --- 查询当前页的帖子列表,实际中应只查询已发布的帖子 ---
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, slug, title, content, created_at AS "created_at!", updated_at AS "updated_at!", published_at 
            FROM posts 
            -- WHERE published_at IS NOT NULL AND published_at <= NOW() -- 过滤已发布的
            ORDER BY created_at DESC -- 或者 ORDER BY published_at DESC
            limit $1 offset $2
            "#,   // 注意: 在实际应用中可能需要加上 WHERE published_at IS NOT NULL AND published_at <= NOW() ORDER BY published_at DESC
            limit,
            offset
        )
            .fetch_all(&self.pool)
            .await
            .context("查询帖子列表分页数据失败")?;

        // --- 查询总帖子数 ---
        // 注意：COUNT 的过滤条件应与上面 SELECT 的过滤条件一致
        let total_count_result = sqlx::query!(
            r#"
            select count(*) as "count!" from posts
             -- WHERE published_at IS NOT NULL AND published_at <= NOW() -- 同样需要过滤
            "#
        )
        .fetch_one(&self.pool)
        .await
        .context("查询帖子总数失败")?;

        // 从结果中获取 count 值，注意类型可能需要转换
        // query! 返回的 count 通常是 i64 或 Decimal，取决于数据库和 COUNT(*) 的结果
        // 我们在 SELECT COUNT(*) as "count!" 中加了 ! 断言它非空
        let total_items: i64 = total_count_result.count;

        Ok((posts, total_items))
    }

    async fn update(
        &self,
        id: Uuid,
        payload: &UpdatePostPayload,
        new_slug_opt: Option<&str>,
    ) -> Result<Post> {
        // 开启数据库事务
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to begin transaction for updating post")?;

        // 1. 获取当前帖子数据 current_post,准备更新值的逻辑
        let current_post = self
            .get_by_id(id)
            .await
            .context(format!("更新操作：获取 Post (id: {})", id))?
            .ok_or_else(|| anyhow::anyhow!("更新目标 Post (id: {}) 未找到，无法继续", id))?;

        // 准备要更新的字段值
        // 对于 title, content, slug，如果 payload 中是 None，则使用 current_post 的旧值
        let title_to_update = payload.title.as_deref().unwrap_or(&current_post.title);
        let content_to_update = payload.content.as_deref().unwrap_or(&current_post.content);
        // Slug处理： 优先使用 service 层根据 title 变化生成并传入的 new_slug
        // 其次使用 payload 中直接指定的 slug (如果 service 层没传 new_slug)
        // 最后才保持 current_post.slug 不变
        let slug_to_update = match new_slug_opt {
            Some(s_from_service) => s_from_service,
            None => match &payload.slug {
                Some(s_from_payload) => s_from_payload.as_str(),
                None => &current_post.slug,
            },
        };
        // 总是更新 updated_at
        let now = Utc::now();

        let published_at_to_update;

        if payload.unpublish {
            published_at_to_update = None;
        } else if let Some(new_published_val) = payload.published_at {
            published_at_to_update = Some(new_published_val);
        } else {
            published_at_to_update = current_post.published_at;
        }

        // 2. 更新帖子基本信息
        let updated_post_from_db = sqlx::query_as!(
            Post,
            r#"
            update posts
            set title = $1,content = $2,slug = $3,updated_at = $4,published_at = $5
            where id = $6
            returning id, slug, title, content, created_at AS "created_at!", updated_at AS "updated_at!", published_at
            "#,
            title_to_update,
            content_to_update,
            slug_to_update,
            now,
            published_at_to_update,
            id
        )
            .fetch_one(&mut *txn)
            .await
            .context(format!("数据库层面更新 Post (id: {}) 失败", id))?;

        // 3. 更新分类关联（如果payload中提供了category_ids）
        // is_update is true
        if payload.category_ids.is_some() {
            // 只有当 payload 中明确包含 category_ids 时才操作
            Self::manage_post_categories(
                &mut txn,
                updated_post_from_db.id,
                &payload.category_ids,
                true,
            )
            .await
            .context("Failed to manage post categories during update")?;
        }

        // 4. 更新标签关联（如果 payload 中提供了 tag_ids)
        // is_update is true
        if payload.tag_ids.is_some() {
            // 只有当 payload 中明确包含 tag_ids 时才操作
            Self::manage_post_tags(&mut txn, updated_post_from_db.id, &payload.tag_ids, true)
                .await
                .context("Failed to manage post tags during update")?;
        }

        // 提交事务
        txn.commit()
            .await
            .context("Failed to commit transaction for updating post")?;

        Ok(updated_post_from_db)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        // 删除帖子时，由于 post_categories 和 post_tags 表设置了 ON DELETE CASCADE,
        // 中间表的关联记录会自动被删除。所以这里只需要删除 posts 表的记录。
        let result = sqlx::query!("delete from posts where id = $1", id)
            .execute(&self.pool)
            .await
            .context(format!("删除 Post (id: {})", id))?;

        if result.rows_affected() == 0 {
            anyhow::bail!("尝试删除 Post (id: {}) 时未找到记录", id);
        }
        Ok(())
    }

    // 实现获取帖子关联的分类和标签的方法
    async fn get_categories_for_post(&self, post_id: Uuid) -> Result<Vec<CategoryDto>> {
        let categories = sqlx::query_as!(
            CategoryDto,
            r#"
            select c.id,c.name,c.slug
            from categories c
            inner join post_categories pc on c.id = pc.category_id
            where pc.post_id = $1
            order by c.name
            "#,
            post_id
        )
        .fetch_all(&self.pool)
        .await
        .context(format!("Failed to fetch categories for post {}", post_id))?;

        Ok(categories)
    }

    async fn get_tags_for_post(&self, post_id: Uuid) -> Result<Vec<TagDto>> {
        let tags = sqlx::query_as!(
            TagDto,
            r#"
            select t.id,t.name,t.slug
            from tags t
            inner join post_tags pt on t.id = pt.tag_id
            where pt.post_id = $1
            order by t.name
            "#,
            post_id
        )
        .fetch_all(&self.pool)
        .await
        .context(format!("Failed to fetch tags for post {}", post_id))?;

        Ok(tags)
    }
}
