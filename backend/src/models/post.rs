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

