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
    pub author_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>, // 使用Option表示文章可能为草稿状态

    // 草稿分享相关字段
    #[sqlx(default)]
    pub draft_shared_with: Option<Vec<Uuid>>, // 分享给哪些用户（UUID数组）
    #[sqlx(default)]
    pub is_draft_public: Option<bool>, // 是否允许有权限的编辑查看
}

// 草稿访问日志模型
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct DraftAccessLog {
    pub id: Uuid,
    pub post_id: Uuid,
    pub accessed_by: Uuid,
    pub access_type: String,           // 'view', 'edit', 'delete'
    pub access_reason: Option<String>, // 访问原因说明
    pub created_at: DateTime<Utc>,
}
