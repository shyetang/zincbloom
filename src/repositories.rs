use crate::models::{CreatePostPayload, Post, UpdatePostPayload};
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

// 定义仓库操作的 trait
#[async_trait]
pub trait PostRepository: Send + Sync {
    // Send + Sync 是因为要在async Axum中共享
    async fn create(&self, payload: &CreatePostPayload, slug: &str) -> Result<Post>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Post>>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<Post>>;
    async fn list(&self,limit: i64,offset: i64) -> Result<(Vec<Post>,i64)>;

    async fn update(
        &self,
        id: Uuid,
        payload: &UpdatePostPayload,
        new_slug: Option<&str>,
    ) -> Result<Post>;

    async fn delete(&self, id: Uuid) -> Result<()>;
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
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn create(&self, payload: &CreatePostPayload, slug: &str) -> Result<Post> {
        let post_id = Uuid::new_v4();
        let now = Utc::now();
        let post = sqlx::query_as!(
            Post,
            r#"
            insert into posts (id,slug,title,content,created_at,updated_at,published_at)
            values ($1,$2,$3,$4,$5,$6,$7)
            returning id,slug,title,content,created_at,updated_at,published_at
            "#,
            post_id,
            slug,
            payload.title,
            payload.content,
            now,
            now,
            None::<DateTime<Utc>>
        )
        .fetch_one(&self.pool)
        .await
        .context("创建 Post 记录 失败 (INSERT)")?; // 使用 ？ 和 context
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
            select id,slug,title,content,created_at as "created_at!",updated_at as "updated_at!",published_at from posts where slug = $1
            "#,
            slug
        )
            .fetch_optional(&self.pool)
            .await
            .context(format!("按 slug ({}) 查询 Post 失败", slug))?;
        Ok(post)
    }

    async fn list(&self,limit: i64,offset:i64) -> Result<(Vec<Post>,i64)> {
        // // --- 查询当前页的帖子列表,实际中应只查询已发布的帖子 ---
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
        
        Ok((posts,total_items))
    }

    async fn update(
        &self,
        id: Uuid,
        payload: &UpdatePostPayload,
        new_slug: Option<&str>,
    ) -> Result<Post> {
        // 获取 current_post,准备更新值的逻辑
        let current_post = self
            .get_by_id(id)
            .await
            .context(format!("更新前未找到 Post (id: {})", id))?
            .ok_or_else(|| {
                anyhow::anyhow!("更新前未找到 Post (id: {}) 但查询成功返回 None？", id)
            })?;

        let title_to_update = payload.title.as_deref().unwrap_or(&current_post.title);
        let content_to_update = payload.content.as_deref().unwrap_or(&current_post.content);
        let slug_to_update = new_slug.unwrap_or(&current_post.slug);
        let now = Utc::now();

        let update_post = sqlx::query_as!(
            Post,
            r#"
            update posts set title = $1,content = $2,slug = $3,updated_at = $4 where id = $5
            returning id, slug, title, content, created_at AS "created_at!", updated_at AS "updated_at!", published_at
            "#,
            title_to_update,
            content_to_update,
            slug_to_update,
            now,
            id
        )
            .fetch_one(&self.pool)
            .await
            .context(format!("更新 Post (id: {}) 失败",id))?;
        Ok(update_post)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query!("delete from posts where id = $1", id)
            .execute(&self.pool)
            .await
            .context(format!("删除 Post (id: {})", id))?;

        if result.rows_affected() == 0 {
            anyhow::bail!("尝试删除 Post (id: {}) 时未找到记录", id);
        }
        Ok(())
    }
}
