use serde::{Deserialize, Serialize};

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