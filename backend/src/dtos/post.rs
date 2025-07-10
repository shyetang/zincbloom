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

    // 草稿分享相关字段
    #[serde(default)]
    pub draft_shared_with: Option<Vec<Uuid>>, // 分享给哪些用户
    #[serde(default)]
    pub is_draft_public: Option<bool>, // 是否允许有权限的编辑查看
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

    // 草稿分享相关字段
    #[serde(default)]
    pub draft_shared_with: Option<Vec<Uuid>>, // 更新分享给的用户列表
    #[serde(default)]
    pub is_draft_public: Option<bool>, // 更新是否允许编辑查看
}

/// 草稿分享操作DTO
#[derive(Debug, Deserialize, Serialize)]
pub struct ShareDraftPayload {
    pub shared_with: Vec<Uuid>,  // 要分享给的用户ID列表
    pub is_public: bool,         // 是否设为公开（允许有权限的编辑查看）
    pub message: Option<String>, // 分享时的消息
}

/// 草稿访问日志DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct DraftAccessLogDto {
    pub id: Uuid,
    pub post_id: Uuid,
    pub post_title: String,
    pub accessed_by: Uuid,
    pub accessed_by_username: String,
    pub access_type: String,
    pub access_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 用户基本信息DTO，用于在文章中显示作者信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserBasicDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
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
    pub author_id: Option<Uuid>,  // 保留以兼容现有代码
    pub author: Option<UserBasicDto>, // 新增作者详细信息
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    // 关联的分类信息
    pub categories: Option<Vec<CategoryDto>>,
    // 关联的标签信息
    pub tags: Option<Vec<TagDto>>,

    // 草稿分享信息（仅对草稿有效，且仅对有权查看的用户显示）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_shared_with: Option<Vec<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_accessing_others_draft: Option<bool>, // 标识是否在访问他人的草稿
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
