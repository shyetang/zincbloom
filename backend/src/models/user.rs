use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// 用户 结构体
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[sqlx(rename = "hashed_password")] // 确保与数据库列名匹配
    #[serde(skip_serializing)] // 在序列化为JSON时跳过此字段，避免密码哈希泄露
    pub hashed_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email_verified_at: Option<DateTime<Utc>>,
}

// 用于API响应的用户公开信息结构体，不包含密码等敏感数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPublic {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub roles: Vec<String>, // 用户拥有的角色名称列表
}
