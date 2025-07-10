use serde::{Deserialize, Serialize};
use uuid::Uuid;

// 用于创建新分类的请求体结构
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryPayload {
    pub name: Option<String>,
}

// 用于更新分类的请求体结构
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateCategoryPayload {
    // 所有字段都是可选的，表示只更新提供的字段
    // slug通常根据name变化自动更新
    pub name: Option<String>,
}

// === 管理员专用的DTO ===

/// 分类合并请求
#[derive(Debug, Serialize, Deserialize)]
pub struct MergeCategoriesPayload {
    pub target_category_id: Uuid,       // 目标分类ID（保留的分类）
    pub source_category_ids: Vec<Uuid>, // 源分类ID列表（将被合并删除的分类）
}

/// 批量删除分类请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteCategoriesPayload {
    pub category_ids: Vec<Uuid>,
}
