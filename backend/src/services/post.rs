use crate::dtos::post::{PaginatedResponse, Pagination};
use crate::models::Post;
use crate::models::post::{CreatePostPayload, UpdatePostPayload};
use crate::repositories::PostRepository;
use anyhow::{Context, Result, anyhow};
use slug::slugify;
use std::sync::Arc;
use uuid::Uuid;

// Post服务结构体，持有仓库的引用（使用Arc<dyn Trait>支持多态和共享）
#[derive(Clone)]
pub struct PostService {
    repo: Arc<dyn PostRepository>,
}

impl PostService {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_post(&self, payload: CreatePostPayload) -> Result<Post> {
        if payload.title.trim().is_empty() {
            // 使用 anyhow！ 宏创建错误
            return Err(anyhow!("标题不能为空"));
        }
        if payload.content.trim().is_empty() {
            return Err(anyhow!("内容不能为空"));
        }
        // 根据文章 title 生成 slug
        let slug = slugify(&payload.title);

        // 调用仓库，使用 ? 和 context
        let post = self
            .repo
            .create(&payload, &slug)
            .await
            .context("Service未能创建帖子")?;

        // 在 anyhow 中，处理唯一约束冲突需要检查错误的根源 (downcasting)
        // 但更简单的方式是让它作为普通的 anyhow::Error 传递上去
        // 如果需要特定处理，会增加复杂度

        Ok(post)
    }

    pub async fn get_post_by_id(&self, id: Uuid) -> Result<Post> {
        let post_option = self
            .repo
            .get_by_id(id)
            .await
            .context(format!("Service未能通过id: ({})获取帖子", id))?;

        // 将 Option<Post> 转换为 Result<Post, anyhow::Error>
        post_option.ok_or_else(|| anyhow!("未找到id为{}的帖子", id))
    }

    pub async fn get_post_by_slug(&self, slug: &str) -> Result<Post> {
        let post_option = self
            .repo
            .get_by_slug(slug)
            .await
            .context(format!("Service未能通过 slug ({}) 获取帖子", slug))?;
        post_option.ok_or_else(|| anyhow!("未找到 slug 为 '{}' 的帖子", slug))
    }

    pub async fn list_posts(&self, pagination: Pagination) -> Result<PaginatedResponse<Post>> {
        // 从 Pagination DTO 获取验证过的分页参数
        let limit = pagination.limit();
        let offset = pagination.offset();
        let page = pagination.page();
        let page_size = pagination.page_size();

        // 调用仓库方法
        let (posts, total_items) = self
            .repo
            .list(limit, offset)
            .await
            .context("Service 未能获取分页的帖子列表")?;

        let response = PaginatedResponse::new(posts, total_items, page, page_size);

        Ok(response)
    }

    pub async fn update_post(&self, id: Uuid, payload: UpdatePostPayload) -> Result<Post> {
        let maybe_new_slug = if let Some(new_title) = &payload.title {
            if !new_title.trim().is_empty() {
                Some(slugify(new_title))
            } else {
                return Err(anyhow!("标题不能为空"));
            }
        } else {
            None
        };
        self.repo
            .update(id, &payload, maybe_new_slug.as_deref())
            .await
            .context(format!("Service 未能更新帖子 (id: {})", id))
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<()> {
        self.repo
            .delete(id)
            .await
            .context(format!("Service未能删除帖子 (id: {})", id))
    }
}
