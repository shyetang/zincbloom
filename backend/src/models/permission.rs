use chrono::{DateTime, Utc};
use uuid::Uuid;

// 权限 结构体
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>, // 权限描述 (可选)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
