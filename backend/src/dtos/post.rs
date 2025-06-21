use crate::models::{Category, Tag};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 用于创建新文章的数据结构（DTO - Data Transfer Object）
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePostPayload {
    pub title: String,
    pub content: String,
    // 帖子的分类 ID 列表 (Option<Vec<Uuid>> 表示可以不提供，或者提供一个空的 Vec)
    pub category_ids: Option<Vec<Uuid>>,
    pub tag_ids: Option<Vec<Uuid>>, // 帖子的标签 ID 列表
                                    // 注意：创建时通常不需要 slug 和 published_at，slug 应自动生成，published_at 应为 None
}

/// 用于更新文章的数据结构（DTO）
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdatePostPayload {
    pub title: Option<String>, // 使用 Option 表示可选更新
    pub content: Option<String>,
    pub slug: Option<String>,
    // Option<Vec<Uuid>>: 如果是 None，则不更新此关联；
    // 如果是 Some(Vec)，则将关联设置为该 Vec (可以是空 Vec 表示清除所有关联)
    pub category_ids: Option<Vec<Uuid>>,
    pub tag_ids: Option<Vec<Uuid>>,
    // 用于设置或更改发布时间
    pub published_at: Option<DateTime<Utc>>, // Option<Option<...>> 允许设置为 NULL
    // 明确的标志来指示是否要撤销发布 (将 published_at 置为 NULL)
    // serde(default) 使得如果 JSON 中不提供 unpublish，它默认为 false。
    #[serde(default)]
    pub unpublish: bool,
}

/// 用于在获取单个帖子详情时，同时返回帖子的基本信息及其关联的分类和标签信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostDetailDto {
    // Post 模型中的字段
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub content_markdown: String, // 用于存放原始 Markdown
    pub content_html: String,     // 用于存放渲染后的 HTML
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    // 关联的分类信息
    pub categories: Option<Vec<CategoryDto>>,
    // 关联的标签信息
    pub tags: Option<Vec<TagDto>>,
}

/// 分类的简化 DTO,不想在 PostDetailDto 中暴露完整的 Category 模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryDto {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

impl From<Category> for CategoryDto {
    fn from(category: Category) -> Self {
        Self {
            id: category.id,
            name: category.name,
            slug: category.slug,
        }
    }
}

/// 标签的简化 DTO
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagDto {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

impl From<Tag> for TagDto {
    fn from(tag: Tag) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
            slug: tag.slug,
        }
    }
}
