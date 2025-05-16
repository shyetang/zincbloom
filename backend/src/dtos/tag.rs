use serde::{Deserialize, Serialize};

/// 用于创建新标签的请求体结构
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagPayload {
    pub name: String, // 创建标签时只需要提供名称，slug 将在后端自动生成
}

/// 用于更新标签的请求体结构
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateTagPayload {
    pub name: Option<String>,   // name 是可选的，表示只更新提供的字段
                                // slug 通常不直接由用户更新，而是根据 name 的变化在后端自动更新
}
