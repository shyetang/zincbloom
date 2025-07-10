use crate::dtos::post::{
    CategoryDto, CreatePostPayload, DraftAccessLogDto, ShareDraftPayload, TagDto, UpdatePostPayload,
};
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
    async fn create(
        &self,
        author_id: Uuid,
        payload: &CreatePostPayload,
        slug: &str,
    ) -> Result<Post>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Post>>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<Post>>;
    async fn list(&self, limit: i64, offset: i64) -> Result<(Vec<Post>, i64)>;
    // 新增：按作者ID过滤的文章列表
    async fn list_by_author(
        &self,
        author_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Post>, i64)>;
    // 新增：查询已发布的文章列表（公开接口）
    async fn list_published(
        &self,
        limit: i64,
        offset: i64,
        include_banned: bool,
    ) -> Result<(Vec<Post>, i64)>;
    // 新增：根据ID获取已发布的文章
    async fn get_published_by_id(&self, id: Uuid, include_banned: bool) -> Result<Option<Post>>;
    // 新增：根据slug获取已发布的文章
    async fn get_published_by_slug(&self, slug: &str, include_banned: bool)
    -> Result<Option<Post>>;

    async fn update(
        &self,
        id: Uuid,
        payload: &UpdatePostPayload,
        new_slug: Option<&str>,
    ) -> Result<Post>;

    async fn delete(&self, id: Uuid) -> Result<()>;

    // 新增：发布文章
    async fn publish(&self, id: Uuid) -> Result<()>;

    // 新增：撤回文章
    async fn unpublish(&self, id: Uuid) -> Result<()>;

    // 获取帖子的完整分类和标签对象
    async fn get_categories_for_post(&self, post_id: Uuid) -> Result<Vec<CategoryDto>>;
    async fn get_tags_for_post(&self, post_id: Uuid) -> Result<Vec<TagDto>>;
    // 获取帖子的分类和标签IDs
    // 暂时不实现，但作为未来获取关联信息的占位
    // async fn get_category_ids_for_post(&self, post_id: Uuid, pool: &PgPool) -> Result<Vec<Uuid>>;
    // async fn get_tag_ids_for_post(&self, post_id: Uuid, pool: &PgPool) -> Result<Vec<Uuid>>;

    // 获取帖子的作者id
    async fn get_author_id(&self, post_id: Uuid) -> Result<Option<Uuid>>;

    // ================================
    // 草稿权限和分享相关方法
    // ================================

    // 检查用户是否可以访问草稿
    async fn can_access_draft(&self, post_id: Uuid, user_id: Uuid) -> Result<bool>;

    // 分享草稿给指定用户
    async fn share_draft(&self, post_id: Uuid, payload: &ShareDraftPayload) -> Result<()>;

    // 获取用户可以访问的草稿列表（包括自己的和分享给自己的）
    async fn list_accessible_drafts(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Post>, i64)>;

    // 记录草稿访问日志
    async fn log_draft_access(
        &self,
        post_id: Uuid,
        accessed_by: Uuid,
        access_type: &str,
        reason: Option<&str>,
    ) -> Result<()>;

    // 获取草稿访问日志
    async fn get_draft_access_logs(
        &self,
        post_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DraftAccessLogDto>>;

    // 检查文章是否为草稿
    async fn is_draft(&self, post_id: Uuid) -> Result<bool>;

    // 根据用户权限获取可访问的文章列表（用于管理界面）
    async fn list_posts_with_access_control(
        &self,
        user_id: Uuid,
        can_read_any: bool,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Post>, i64)>;

    // ================================
    // 文章封禁相关方法
    // ================================

    // 封禁文章
    async fn ban_post(&self, post_id: Uuid) -> Result<()>;

    // 解封文章
    async fn unban_post(&self, post_id: Uuid) -> Result<()>;

    // 检查文章是否被封禁
    async fn is_banned(&self, post_id: Uuid) -> Result<bool>;
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
    async fn create(
        &self,
        author_id: Uuid,
        payload: &CreatePostPayload,
        slug: &str,
    ) -> Result<Post> {
        let post_id = Uuid::new_v4();
        let now = Utc::now();

        // 开始数据库事务
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to begin transaction for creating post")?;

        // 1. 插入帖子基本信息（包括草稿分享字段）
        let post = sqlx::query_as!(
            Post,
            r#"
            insert into posts (id,slug,title,content,created_at,updated_at,published_at,author_id,draft_shared_with,is_draft_public,is_banned)
            values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
            returning id,slug,title,content,created_at,updated_at,published_at,author_id,draft_shared_with,is_draft_public,is_banned
            "#,
            post_id,
            slug,
            &payload.title,
            &payload.content,
            now,
            now,
            None::<DateTime<Utc>>,
            author_id,
            payload.draft_shared_with.as_deref(),
            payload.is_draft_public,
            false // 默认不封禁
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
            select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at,author_id,draft_shared_with,is_draft_public,is_banned from posts where id = $1
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
            select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at,author_id,draft_shared_with,is_draft_public,is_banned
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
            SELECT id, slug, title, content, created_at AS "created_at!", updated_at AS "updated_at!", published_at,author_id,draft_shared_with,is_draft_public,is_banned 
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

    async fn list_by_author(
        &self,
        author_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Post>, i64)> {
        // --- 查询当前页的帖子列表,只返回指定作者的文章 ---
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, slug, title, content, created_at AS "created_at!", updated_at AS "updated_at!", published_at, author_id,draft_shared_with,is_draft_public,is_banned 
            FROM posts 
            WHERE author_id = $1
            -- WHERE published_at IS NOT NULL AND published_at <= NOW() -- 过滤已发布的
            ORDER BY created_at DESC -- 或者 ORDER BY published_at DESC
            limit $2 offset $3
            "#,
            author_id,
            limit,
            offset
        )
            .fetch_all(&self.pool)
            .await
            .context("查询指定作者的帖子列表分页数据失败")?;

        // --- 查询指定作者的总帖子数 ---
        let total_count_result = sqlx::query!(
            r#"
            select count(*) as "count!" from posts
            WHERE author_id = $1
             -- WHERE published_at IS NOT NULL AND published_at <= NOW() -- 同样需要过滤
            "#,
            author_id
        )
        .fetch_one(&self.pool)
        .await
        .context("查询指定作者的帖子总数失败")?;

        let total_items: i64 = total_count_result.count;

        Ok((posts, total_items))
    }

    async fn list_published(
        &self,
        limit: i64,
        offset: i64,
        include_banned: bool,
    ) -> Result<(Vec<Post>, i64)> {
        // --- 查询已发布的文章列表 ---
        let posts = if include_banned {
            // 管理员可以看到被封禁的文章
            sqlx::query_as!(
                Post,
                r#"
                SELECT id, slug, title, content, created_at AS "created_at!", updated_at AS "updated_at!", published_at, author_id,draft_shared_with,is_draft_public,is_banned 
                FROM posts 
                WHERE published_at IS NOT NULL AND published_at <= NOW()
                ORDER BY published_at DESC
                limit $1 offset $2
                "#,
                limit,
                offset
            )
                .fetch_all(&self.pool)
                .await
                .context("查询已发布文章列表分页数据失败")?
        } else {
            // 普通用户只能看到未被封禁的文章
            sqlx::query_as!(
                Post,
                r#"
                SELECT id, slug, title, content, created_at AS "created_at!", updated_at AS "updated_at!", published_at, author_id,draft_shared_with,is_draft_public,is_banned 
                FROM posts 
                WHERE published_at IS NOT NULL AND published_at <= NOW() AND (is_banned = false OR is_banned IS NULL)
                ORDER BY published_at DESC
                limit $1 offset $2
                "#,
                limit,
                offset
            )
                .fetch_all(&self.pool)
                .await
                .context("查询已发布文章列表分页数据失败")?
        };

        // --- 查询已发布文章总数 ---
        let total_count_result = if include_banned {
            sqlx::query_scalar!(
                r#"
                select count(*) from posts
                WHERE published_at IS NOT NULL AND published_at <= NOW()
                "#
            )
            .fetch_one(&self.pool)
            .await
            .context("查询已发布文章总数失败")?
        } else {
            sqlx::query_scalar!(
                r#"
                select count(*) from posts
                WHERE published_at IS NOT NULL AND published_at <= NOW() AND (is_banned = false OR is_banned IS NULL)
                "#
            )
            .fetch_one(&self.pool)
            .await
            .context("查询已发布文章总数失败")?
        };

        let total_items: i64 = total_count_result.unwrap_or(0);

        Ok((posts, total_items))
    }

    async fn get_published_by_id(&self, id: Uuid, include_banned: bool) -> Result<Option<Post>> {
        let post = if include_banned {
            sqlx::query_as!(
                Post,
                r#"
                select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at,author_id,draft_shared_with,is_draft_public,is_banned 
                from posts 
                where id = $1 AND published_at IS NOT NULL AND published_at <= NOW()
                "#,
                id
            )
                .fetch_optional(&self.pool)
                .await
                .context(format!("按 id ({}) 查询已发布 Post 失败", id))?
        } else {
            sqlx::query_as!(
                Post,
                r#"
                select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at,author_id,draft_shared_with,is_draft_public,is_banned 
                from posts 
                where id = $1 AND published_at IS NOT NULL AND published_at <= NOW() AND (is_banned = false OR is_banned IS NULL)
                "#,
                id
            )
                .fetch_optional(&self.pool)
                .await
                .context(format!("按 id ({}) 查询已发布 Post 失败", id))?
        };
        Ok(post)
    }

    async fn get_published_by_slug(
        &self,
        slug: &str,
        include_banned: bool,
    ) -> Result<Option<Post>> {
        let post = if include_banned {
            sqlx::query_as!(
                Post,
                r#"
                select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at,author_id,draft_shared_with,is_draft_public,is_banned
                from posts 
                where slug = $1 AND published_at IS NOT NULL AND published_at <= NOW()
                "#,
                slug
            )
                .fetch_optional(&self.pool)
                .await
                .context(format!("按 slug ({}) 查询已发布 Post 失败", slug))?
        } else {
            sqlx::query_as!(
                Post,
                r#"
                select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at,author_id,draft_shared_with,is_draft_public,is_banned
                from posts 
                where slug = $1 AND published_at IS NOT NULL AND published_at <= NOW() AND (is_banned = false OR is_banned IS NULL)
                "#,
                slug
            )
                .fetch_optional(&self.pool)
                .await
                .context(format!("按 slug ({}) 查询已发布 Post 失败", slug))?
        };
        Ok(post)
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

        // 处理草稿分享字段
        let draft_shared_with_to_update = payload
            .draft_shared_with
            .as_deref()
            .unwrap_or(current_post.draft_shared_with.as_deref().unwrap_or(&[]));
        let is_draft_public_to_update = payload
            .is_draft_public
            .unwrap_or(current_post.is_draft_public.unwrap_or(false));

        // 2. 更新帖子基本信息（包括草稿分享字段）
        let updated_post_from_db = sqlx::query_as!(
            Post,
            r#"
            update posts
            set title = $1,content = $2,slug = $3,updated_at = $4,published_at = $5,draft_shared_with = $6,is_draft_public = $7
            where id = $8
            returning id, slug, title, content, created_at AS "created_at!", updated_at AS "updated_at!", published_at,author_id,draft_shared_with,is_draft_public,is_banned
            "#,
            title_to_update,
            content_to_update,
            slug_to_update,
            now,
            published_at_to_update,
            draft_shared_with_to_update,
            is_draft_public_to_update,
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

    async fn get_author_id(&self, post_id: Uuid) -> Result<Option<Uuid>> {
        let result = sqlx::query!("select author_id from posts where id = $1", post_id)
            .fetch_optional(&self.pool)
            .await
            .context(format!(
                "从数据库获取 Post id 为 {} 的 author id失败",
                post_id
            ))?;
        Ok(result.and_then(|record| record.author_id))
    }

    // 新增：发布文章
    async fn publish(&self, id: Uuid) -> Result<()> {
        let now = Utc::now();
        let result = sqlx::query!(
            "UPDATE posts SET published_at = $1, updated_at = $1 WHERE id = $2",
            now,
            id
        )
        .execute(&self.pool)
        .await
        .context(format!("发布文章 (id: {}) 失败", id))?;

        if result.rows_affected() == 0 {
            anyhow::bail!("尝试发布文章 (id: {}) 时未找到记录", id);
        }

        tracing::info!("文章 {} 已发布", id);
        Ok(())
    }

    // 新增：撤回文章
    async fn unpublish(&self, id: Uuid) -> Result<()> {
        let now = Utc::now();
        let result = sqlx::query!(
            "UPDATE posts SET published_at = NULL, updated_at = $1 WHERE id = $2",
            now,
            id
        )
        .execute(&self.pool)
        .await
        .context(format!("撤回文章 (id: {}) 失败", id))?;

        if result.rows_affected() == 0 {
            anyhow::bail!("尝试撤回文章 (id: {}) 时未找到记录", id);
        }

        tracing::info!("文章 {} 已撤回", id);
        Ok(())
    }

    // 检查用户是否可以访问草稿
    async fn can_access_draft(&self, post_id: Uuid, user_id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            SELECT 
                author_id,
                draft_shared_with,
                is_draft_public,
                published_at
            FROM posts 
            WHERE id = $1
            "#,
            post_id
        )
        .fetch_optional(&self.pool)
        .await
        .context(format!("检查草稿访问权限失败，post_id: {}", post_id))?;

        let Some(post) = result else {
            return Ok(false); // 文章不存在
        };

        // 如果文章已发布，那么不是草稿，任何人都可以访问
        if post.published_at.is_some() {
            return Ok(true);
        }

        // 1. 如果是作者本人，直接允许访问
        if post.author_id == Some(user_id) {
            return Ok(true);
        }

        // 2. 检查是否被分享给该用户
        if let Some(shared_users) = post.draft_shared_with {
            if shared_users.contains(&user_id) {
                return Ok(true);
            }
        }

        // 3. 检查是否设为公开（这个需要结合用户权限检查）
        if post.is_draft_public == Some(true) {
            // 这里返回true，但上层应该结合用户的 post:draft:access_shared 权限检查
            return Ok(true);
        }

        Ok(false)
    }

    // 分享草稿给指定用户
    async fn share_draft(&self, post_id: Uuid, payload: &ShareDraftPayload) -> Result<()> {
        // 首先检查文章是否存在且为草稿
        let is_draft = self.is_draft(post_id).await?;
        if !is_draft {
            anyhow::bail!("只能分享草稿文章");
        }

        let result = sqlx::query!(
            r#"
            UPDATE posts 
            SET 
                draft_shared_with = $1,
                is_draft_public = $2,
                updated_at = $3
            WHERE id = $4 AND published_at IS NULL
            "#,
            &payload.shared_with,
            payload.is_public,
            Utc::now(),
            post_id
        )
        .execute(&self.pool)
        .await
        .context(format!("分享草稿失败，post_id: {}", post_id))?;

        if result.rows_affected() == 0 {
            anyhow::bail!("草稿分享失败：文章不存在或已发布");
        }

        tracing::info!(
            "草稿 {} 已分享给 {} 个用户",
            post_id,
            payload.shared_with.len()
        );
        Ok(())
    }

    // 获取用户可以访问的草稿列表（包括自己的和分享给自己的）
    async fn list_accessible_drafts(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Post>, i64)> {
        // 查询用户可以访问的草稿：
        // 1. 自己创建的草稿
        // 2. 分享给自己的草稿
        // 3. 公开的草稿（如果用户有相应权限，这个在上层检查）
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT 
                id, slug, title, content, author_id, created_at, updated_at, published_at,
                draft_shared_with, is_draft_public, is_banned
            FROM posts 
            WHERE published_at IS NULL 
            AND (
                author_id = $1  -- 自己的草稿
                OR $1 = ANY(draft_shared_with)  -- 分享给自己的草稿
                OR (is_draft_public = true)  -- 公开的草稿
            )
            ORDER BY updated_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .context("获取用户可访问的草稿列表失败")?;

        // 获取总数
        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM posts 
            WHERE published_at IS NULL 
            AND (
                author_id = $1  
                OR $1 = ANY(draft_shared_with)  
                OR (is_draft_public = true)
            )
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("获取用户可访问的草稿总数失败")?;

        Ok((posts, total.count.unwrap_or(0)))
    }

    // 记录草稿访问日志
    async fn log_draft_access(
        &self,
        post_id: Uuid,
        accessed_by: Uuid,
        access_type: &str,
        reason: Option<&str>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO draft_access_logs (post_id, accessed_by, access_type, access_reason)
            VALUES ($1, $2, $3, $4)
            "#,
            post_id,
            accessed_by,
            access_type,
            reason
        )
        .execute(&self.pool)
        .await
        .context("记录草稿访问日志失败")?;

        tracing::info!(
            "记录草稿访问日志：用户 {} {} 了草稿 {}",
            accessed_by,
            access_type,
            post_id
        );
        Ok(())
    }

    // 获取草稿访问日志
    async fn get_draft_access_logs(
        &self,
        post_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DraftAccessLogDto>> {
        let logs = if let Some(post_id) = post_id {
            // 获取特定文章的访问日志
            sqlx::query_as!(
                DraftAccessLogDto,
                r#"
                SELECT 
                    dal.id,
                    dal.post_id,
                    p.title as post_title,
                    dal.accessed_by,
                    u.username as accessed_by_username,
                    dal.access_type,
                    dal.access_reason,
                    dal.created_at
                FROM draft_access_logs dal
                JOIN posts p ON dal.post_id = p.id
                JOIN users u ON dal.accessed_by = u.id
                WHERE dal.post_id = $1
                ORDER BY dal.created_at DESC
                LIMIT $2 OFFSET $3
                "#,
                post_id,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await
            .context("获取特定文章的草稿访问日志失败")?
        } else {
            // 获取所有草稿访问日志
            sqlx::query_as!(
                DraftAccessLogDto,
                r#"
                SELECT 
                    dal.id,
                    dal.post_id,
                    p.title as post_title,
                    dal.accessed_by,
                    u.username as accessed_by_username,
                    dal.access_type,
                    dal.access_reason,
                    dal.created_at
                FROM draft_access_logs dal
                JOIN posts p ON dal.post_id = p.id
                JOIN users u ON dal.accessed_by = u.id
                ORDER BY dal.created_at DESC
                LIMIT $1 OFFSET $2
                "#,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await
            .context("获取草稿访问日志失败")?
        };

        Ok(logs)
    }

    // 检查文章是否为草稿
    async fn is_draft(&self, post_id: Uuid) -> Result<bool> {
        let result = sqlx::query!("SELECT published_at FROM posts WHERE id = $1", post_id)
            .fetch_optional(&self.pool)
            .await
            .context(format!("检查文章是否为草稿失败，post_id: {}", post_id))?;

        match result {
            Some(record) => Ok(record.published_at.is_none()),
            None => anyhow::bail!("文章不存在"),
        }
    }

    // 根据用户权限获取可访问的文章列表（用于管理界面）
    async fn list_posts_with_access_control(
        &self,
        user_id: Uuid,
        can_read_any: bool,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Post>, i64)> {
        if can_read_any {
            // 如果用户有权限读取所有文章，则不进行权限过滤
            return self.list(limit, offset).await;
        }

        // 普通用户权限过滤：
        // 1. 自己的所有文章（已发布+草稿）
        // 2. 他人的已发布文章
        // 3. 他人分享给自己的草稿
        // 4. 设为公开的草稿
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT DISTINCT 
                id, slug, title, content, created_at AS "created_at!", updated_at AS "updated_at!", 
                published_at, author_id, draft_shared_with, is_draft_public, is_banned
            FROM posts 
            WHERE 
                author_id = $1  -- 自己的所有文章
                OR published_at IS NOT NULL  -- 他人的已发布文章
                OR (published_at IS NULL AND $1 = ANY(draft_shared_with))  -- 分享给自己的草稿
                OR (published_at IS NULL AND is_draft_public = true)  -- 公开的草稿
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .context("查询用户可访问的文章列表失败")?;

        // 统计总数（使用相同的过滤条件）
        let total_count_result = sqlx::query!(
            r#"
            SELECT COUNT(DISTINCT id) as "count!"
            FROM posts 
            WHERE 
                author_id = $1  -- 自己的所有文章
                OR published_at IS NOT NULL  -- 他人的已发布文章
                OR (published_at IS NULL AND $1 = ANY(draft_shared_with))  -- 分享给自己的草稿
                OR (published_at IS NULL AND is_draft_public = true)  -- 公开的草稿
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .context("查询用户可访问的文章总数失败")?;

        let total_items: i64 = total_count_result.count;

        Ok((posts, total_items))
    }

    // 封禁文章
    async fn ban_post(&self, post_id: Uuid) -> Result<()> {
        sqlx::query!("UPDATE posts SET is_banned = true WHERE id = $1", post_id)
            .execute(&self.pool)
            .await
            .context(format!("封禁文章失败，post_id: {}", post_id))?;

        tracing::info!("文章已被封禁，post_id: {}", post_id);
        Ok(())
    }

    // 解封文章
    async fn unban_post(&self, post_id: Uuid) -> Result<()> {
        sqlx::query!("UPDATE posts SET is_banned = false WHERE id = $1", post_id)
            .execute(&self.pool)
            .await
            .context(format!("解封文章失败，post_id: {}", post_id))?;

        tracing::info!("文章已被解封，post_id: {}", post_id);
        Ok(())
    }

    // 检查文章是否被封禁
    async fn is_banned(&self, post_id: Uuid) -> Result<bool> {
        let result = sqlx::query!("SELECT is_banned FROM posts WHERE id = $1", post_id)
            .fetch_optional(&self.pool)
            .await
            .context(format!("检查文章封禁状态失败，post_id: {}", post_id))?;

        match result {
            Some(record) => Ok(record.is_banned),
            None => anyhow::bail!("文章不存在"),
        }
    }
}
