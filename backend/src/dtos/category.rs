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
    pub target_category_id: Uuid,        // 目标分类ID（保留的分类）
    pub source_category_ids: Vec<Uuid>,  // 源分类ID列表（将被合并删除的分类）
    pub new_target_name: Option<String>, // 可选：给目标分类一个新名称
}

/// 分类合并响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MergeCategoriesResponse {
    pub target_category: crate::models::Category, // 最终的目标分类
    pub merged_category_count: usize,             // 被合并的分类数量
    pub affected_post_count: usize,               // 受影响的文章数量
    pub duplicate_relations_removed: usize,       // 移除的重复关联数量
    pub operation_summary: String,                // 操作摘要
}

/// 批量删除分类请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteCategoriesPayload {
    pub category_ids: Vec<Uuid>,
    pub handle_orphaned_posts: Option<OrphanedPostsHandling>, // 如何处理失去分类的文章
}

/// 批量删除分类响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteCategoriesResponse {
    pub deleted_count: usize,
    pub affected_post_count: usize,
    pub orphaned_post_count: usize, // 完全失去分类的文章数
    pub operation_summary: String,
}

/// 处理孤儿文章的策略
#[derive(Debug, Serialize, Deserialize)]
pub enum OrphanedPostsHandling {
    /// 保持原样（文章可能没有分类）
    LeaveAsIs,
    /// 自动添加"未分类"分类
    AddUncategorizedCategory,
    /// 根据文章内容自动推荐分类
    AutoSuggestCategories,
}

/// 分类合并预览请求
#[derive(Debug, Serialize, Deserialize)]
pub struct MergeCategoriesPreviewPayload {
    pub target_category_id: Uuid,
    pub source_category_ids: Vec<Uuid>,
}

/// 分类合并预览响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MergeCategoriesPreviewResponse {
    pub target_category: crate::models::Category,
    pub source_categories: Vec<crate::models::Category>,
    pub total_posts_affected: usize,
    pub posts_with_duplicates: usize, // 同时拥有源分类和目标分类的文章数
    pub posts_by_category: Vec<CategoryPostCount>, // 每个分类关联的文章数
    pub potential_issues: Vec<String>, // 潜在问题提醒
}

/// 分类文章数量统计
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryPostCount {
    pub category: crate::models::Category,
    pub post_count: usize,
    pub sample_post_titles: Vec<String>, // 示例文章标题（最多5个）
}

/// 分类使用统计
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryUsageStats {
    pub category: crate::models::Category,
    pub post_count: i64,
}

/// 相似分类组（用于管理员清理重复分类）
#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarCategoryGroup {
    pub categories: Vec<crate::models::Category>,
    pub similarity_reason: String, // 例如："名称相似"、"同义词"等
}
