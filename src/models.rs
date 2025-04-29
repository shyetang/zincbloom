use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)] // FromRow 用于从数据库行映射
pub struct Post {
    #[sqlx(rename = "id")]
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>, // 使用Option表示文章可能为草稿状态
}

// 用于创建新文章的数据结构（DTO - Data Transfer Object）
#[derive(Debug, Deserialize)]
pub struct CreatePostPayload {
    pub title: String,
    pub content: String,
    // 注意：创建时通常不需要 slug 和 published_at，slug 应自动生成，published_at 应为 None
}

// 用于更新文章的数据结构（DTO）
#[derive(Debug, Deserialize)]
pub struct UpdatePostPayload {
    pub title: Option<String>, // 使用 Option 表示可选更新
    pub content: Option<String>,
    pub slug: Option<String>,
    pub published_at: Option<Option<DateTime<Utc>>>, // Option<Option<...>> 允许设置为 NULL
}
