use crate::dtos::post::{CreatePostPayload, PostDetailDto, UpdatePostPayload};
use crate::dtos::{PaginatedResponse, Pagination};
use crate::repositories::{CategoryRepository, PostRepository, TagRepository};
use anyhow::{anyhow, Context, Ok, Result};
use slug::slugify;
use std::sync::Arc;
use uuid::Uuid;

// Post服务结构体，持有仓库的引用（使用Arc<dyn Trait>支持多态和共享）
#[derive(Clone)]
pub struct PostService {
    repo: Arc<dyn PostRepository>,
    category_repo: Arc<dyn CategoryRepository>,
    tag_repo: Arc<dyn TagRepository>,
}

impl PostService {
    pub fn new(
        repo: Arc<dyn PostRepository>,
        category_repo: Arc<dyn CategoryRepository>,
        tag_repo: Arc<dyn TagRepository>,
    ) -> Self {
        Self {
            repo,
            category_repo,
            tag_repo,
        }
    }

    // 辅助函数：验证分类 IDs 是否都有效
    async fn validate_category_ids(&self, category_ids: &Option<Vec<Uuid>>) -> Result<()> {
        if let Some(ids) = category_ids {
            for id in ids {
                if self.category_repo.get_by_id(*id).await?.is_none() {
                    return Err(anyhow!("无效的分类 ID：{}", id));
                }
            }
        }
        Ok(())
    }
    // 辅助函数：验证标签 IDs 是否都有效
    async fn validate_tag_ids(&self, tag_ids: &Option<Vec<Uuid>>) -> Result<()> {
        if let Some(ids) = tag_ids {
            for id in ids {
                if self.tag_repo.get_by_id(*id).await?.is_none() {
                    return Err(anyhow!("无效的标签 ID：{}", id));
                }
            }
        }
        Ok(())
    }

    // 创建帖子，并返回包含完整关联信息的 PostDetailDto
    pub async fn create_post(
        &self,
        author_id: Uuid,
        payload: CreatePostPayload,
    ) -> Result<PostDetailDto> {
        if payload.title.trim().is_empty() {
            // 使用 anyhow！ 宏创建错误
            return Err(anyhow!("标题不能为空"));
        }
        if payload.content.trim().is_empty() {
            return Err(anyhow!("内容不能为空"));
        }
        // 验证 category_ids 和 tag_ids
        self.validate_category_ids(&payload.category_ids)
            .await
            .context("创建帖子时验证分类ID失败")?;
        self.validate_tag_ids(&payload.tag_ids)
            .await
            .context("创建帖子时验证标签ID失败")?;

        // 根据文章 title 生成 slug
        let slug = slugify(&payload.title);

        // 调用仓库，repo.create 返回基本的 Post 对象，它已经处理了关联表的写入
        let created_post_basic = self
            .repo
            .create(author_id, &payload, &slug)
            .await
            .context("Service未能创建帖子基本信息及关联")?;

        // 创建成功后，获取完整的 PostDetailDto
        let categories = self
            .repo
            .get_categories_for_post(created_post_basic.id)
            .await
            .context(format!(
                "获取新创建帖子 {} 的分类失败",
                created_post_basic.id
            ))?;
        let tags = self
            .repo
            .get_tags_for_post(created_post_basic.id)
            .await
            .context(format!(
                "获取新创建帖子 {} 的标签失败",
                created_post_basic.id
            ))?;

        let post_detail_dto = PostDetailDto {
            id: created_post_basic.id,
            slug: created_post_basic.slug,
            title: created_post_basic.title,
            content: created_post_basic.content,
            created_at: created_post_basic.created_at,
            updated_at: created_post_basic.created_at,
            published_at: created_post_basic.published_at,
            categories: if categories.is_empty() {
                None
            } else {
                Some(categories)
            },
            tags: if tags.is_empty() { None } else { Some(tags) },
        };
        Ok(post_detail_dto)
    }

    // 获取单个帖子详情，返回 PostDetailDto
    pub async fn get_post_by_id(&self, id: Uuid) -> Result<PostDetailDto> {
        let post = self
            .repo
            .get_by_id(id)
            .await
            .context(format!("Service未能通过id: ({})获取帖子基本信息", id))?
            .ok_or_else(|| anyhow!("未找到 ID 为 {} 的帖子", id))?;

        let categories = self
            .repo
            .get_categories_for_post(post.id)
            .await
            .context(format!("获取新创建帖子 {} 的分类失败", post.id))?;
        let tags = self
            .repo
            .get_tags_for_post(post.id)
            .await
            .context(format!("获取新创建帖子 {} 的标签失败", post.id))?;

        let post_detail_dto = PostDetailDto {
            id: post.id,
            slug: post.slug,
            title: post.title,
            content: post.content,
            created_at: post.created_at,
            updated_at: post.created_at,
            published_at: post.published_at,
            categories: if categories.is_empty() {
                None
            } else {
                Some(categories)
            },
            tags: if tags.is_empty() { None } else { Some(tags) },
        };
        Ok(post_detail_dto)
    }

    // 获取单个帖子详情（通过slug），返回 PostDetailDto
    pub async fn get_post_by_slug(&self, slug: &str) -> Result<PostDetailDto> {
        let post = self
            .repo
            .get_by_slug(slug)
            .await
            .context(format!("Service未能通过 slug ({}) 获取帖子基本信息", slug))?
            .ok_or_else(|| anyhow!("未找到 slug 为 '{}' 的帖子", slug))?;

        let categories = self
            .repo
            .get_categories_for_post(post.id)
            .await
            .context(format!("获取新创建帖子 {} 的分类失败", post.id))?;
        let tags = self
            .repo
            .get_tags_for_post(post.id)
            .await
            .context(format!("获取新创建帖子 {} 的标签失败", post.id))?;

        let post_detail_dto = PostDetailDto {
            id: post.id,
            slug: post.slug,
            title: post.title,
            content: post.content,
            created_at: post.created_at,
            updated_at: post.created_at,
            published_at: post.published_at,
            categories: if categories.is_empty() {
                None
            } else {
                Some(categories)
            },
            tags: if tags.is_empty() { None } else { Some(tags) },
        };
        Ok(post_detail_dto)
    }

    // 获取帖子列表，返回包含 PostDetailDto 的分页响应
    pub async fn list_posts(
        &self,
        pagination: Pagination,
    ) -> Result<PaginatedResponse<PostDetailDto>> {
        // 从 Pagination DTO 获取验证过的分页参数
        let limit = pagination.limit();
        let offset = pagination.offset();
        let page = pagination.page();
        let page_size = pagination.page_size();

        // 1. 获取基本的帖子列表
        let (posts, total_items) = self
            .repo
            .list(limit, offset)
            .await
            .context("Service 未能获取分页的帖子列表(基本信息）")?;

        // 2. 为每个帖子获取其关联的分类和标签 (N+1 查询问题警告)
        let mut post_details_list = Vec::with_capacity(posts.len());

        for post in posts {
            let categories = self
                .repo
                .get_categories_for_post(post.id)
                .await
                .context(format!("获取新创建帖子 {} 的分类失败", post.id))?;
            let tags = self
                .repo
                .get_tags_for_post(post.id)
                .await
                .context(format!("获取新创建帖子 {} 的标签失败", post.id))?;
            let post_detail_dto = PostDetailDto {
                id: post.id,
                slug: post.slug,
                title: post.title,
                content: post.content,
                created_at: post.created_at,
                updated_at: post.created_at,
                published_at: post.published_at,
                categories: if categories.is_empty() {
                    None
                } else {
                    Some(categories)
                },
                tags: if tags.is_empty() { None } else { Some(tags) },
            };
            post_details_list.push(post_detail_dto);
        }

        let response = PaginatedResponse::new(post_details_list, total_items, page, page_size);

        Ok(response)
    }

    // 更新帖子，并返回包含完整关联信息的 PostDetailDto
    pub async fn update_post(&self, id: Uuid, payload: UpdatePostPayload) -> Result<PostDetailDto> {
        // 验证 category_ids 和 tag_ids (如果提供了)
        if payload.category_ids.is_some() {
            self.validate_category_ids(&payload.category_ids)
                .await
                .context("更新帖子时验证分类ID失败")?;
        }
        if payload.tag_ids.is_some() {
            self.validate_tag_ids(&payload.tag_ids)
                .await
                .context("更新帖子时验证标签ID失败")?;
        }

        let maybe_new_slug = if let Some(new_title) = &payload.title {
            if !new_title.trim().is_empty() {
                Some(slugify(new_title))
            } else {
                return Err(anyhow!("标题不能为空"));
            }
        } else {
            None
        };
        // repo.update 返回基本的 Post 对象，它已经处理了关联表的更新
        let post = self
            .repo
            .update(id, &payload, maybe_new_slug.as_deref())
            .await
            .context(format!(
                "Service 未能更新帖子 (id: {}) 的基本信息和关联",
                id
            ))?;
        // 更新成功后，获取完整的 PostDetailDto
        let categories = self
            .repo
            .get_categories_for_post(post.id)
            .await
            .context(format!("获取更新后帖子 {} 的分类失败", post.id))?;
        let tags = self
            .repo
            .get_tags_for_post(post.id)
            .await
            .context(format!("获取更新后帖子 {} 的标签失败", post.id))?;
        let post_detail_dto = PostDetailDto {
            id: post.id,
            slug: post.slug,
            title: post.title,
            content: post.content,
            created_at: post.created_at,
            updated_at: post.created_at,
            published_at: post.published_at,
            categories: if categories.is_empty() {
                None
            } else {
                Some(categories)
            },
            tags: if tags.is_empty() { None } else { Some(tags) },
        };

        Ok(post_detail_dto)
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<()> {
        self.repo
            .delete(id)
            .await
            .context(format!("Service未能删除帖子 (id: {})", id))
    }
    
    // 获取 post 的作者 id
    pub async fn get_post_author(&self,post_id: Uuid) -> Result<Option<Uuid>> {
        self.repo.get_author_id(post_id).await.context("Service层获取作者ID失败")
    }
}
