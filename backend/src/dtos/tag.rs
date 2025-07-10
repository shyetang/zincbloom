use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 用于创建新标签的请求体结构
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagPayload {
    pub name: String, // 创建标签时只需要提供名称，slug 将在后端自动生成
}

/// 用于更新标签的请求体结构
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateTagPayload {
    pub name: Option<String>, // name 是可选的，表示只更新提供的字段
                              // slug 通常不直接由用户更新，而是根据 name 的变化在后端自动更新
}

// === 管理员专用的DTO ===

/// 标签合并请求
#[derive(Debug, Serialize, Deserialize)]
pub struct MergeTagsPayload {
    pub target_tag_id: Uuid,       // 目标标签ID（保留的标签）
    pub source_tag_ids: Vec<Uuid>, // 源标签ID列表（将被合并删除的标签）
    pub new_target_name: Option<String>, // 可选：给目标标签一个新名称
}

/// 标签合并响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MergeTagsResponse {
    pub target_tag: crate::models::Tag, // 最终的目标标签
    pub merged_tag_count: usize,         // 被合并的标签数量
    pub affected_post_count: usize,      // 受影响的文章数量
    pub duplicate_relations_removed: usize, // 移除的重复关联数量
    pub operation_summary: String,       // 操作摘要
}

/// 批量删除标签请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteTagsPayload {
    pub tag_ids: Vec<Uuid>,
    pub handle_orphaned_posts: Option<OrphanedPostsHandling>, // 如何处理失去标签的文章
}

/// 批量删除标签响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteTagsResponse {
    pub deleted_count: usize,
    pub affected_post_count: usize,
    pub orphaned_post_count: usize, // 完全失去标签的文章数
    pub operation_summary: String,
}

/// 处理孤儿文章的策略
#[derive(Debug, Serialize, Deserialize)]
pub enum OrphanedPostsHandling {
    /// 保持原样（文章可能没有标签）
    LeaveAsIs,
    /// 自动添加"未分类"标签
    AddUncategorizedTag,
    /// 根据文章内容自动推荐标签
    AutoSuggestTags,
}

/// 标签合并预览请求
#[derive(Debug, Serialize, Deserialize)]
pub struct MergeTagsPreviewPayload {
    pub target_tag_id: Uuid,
    pub source_tag_ids: Vec<Uuid>,
}

/// 标签合并预览响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MergeTagsPreviewResponse {
    pub target_tag: crate::models::Tag,
    pub source_tags: Vec<crate::models::Tag>,
    pub total_posts_affected: usize,
    pub posts_with_duplicates: usize, // 同时拥有源标签和目标标签的文章数
    pub posts_by_tag: Vec<TagPostCount>, // 每个标签关联的文章数
    pub potential_issues: Vec<String>, // 潜在问题提醒
}

/// 标签文章数量统计
#[derive(Debug, Serialize, Deserialize)]
pub struct TagPostCount {
    pub tag: crate::models::Tag,
    pub post_count: usize,
    pub sample_post_titles: Vec<String>, // 示例文章标题（最多5个）
}
