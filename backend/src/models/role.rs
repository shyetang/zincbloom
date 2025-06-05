use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// 角色 结构体
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub name: String,                // 角色名称 (如 "admin", "user")
    pub description: Option<String>, // 角色描述 (可选)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
