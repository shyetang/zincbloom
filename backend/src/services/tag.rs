use crate::dtos::tag::{CreateTagPayload, UpdateTagPayload};
use crate::models::Tag;
use crate::repositories::TagRepository;
use anyhow::{Context, Result, anyhow};
use slug::slugify;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)] // 需要 Clone，以便在 AppState 中共享
pub struct TagService {
    repo: Arc<dyn TagRepository>, // 依赖 TagRepository trait
}

impl TagService {
    // 构造函数
    pub fn new(repo: Arc<dyn TagRepository>) -> Self {
        Self { repo }
    }

    /// 创建一个新的标签
    pub async fn create_tag(&self, payload: CreateTagPayload) -> Result<Tag> {
        // 1. 验证输入（例如：名称不能为空）
        let name = payload.name.trim();
        if name.is_empty() {
            return Err(anyhow!("标签名称不能为空"));
        }

        // 2. 生成 slug
        let slug = slugify(name);

        // 3. （可选）检查名称或 slug 是否已存在。
        //    如果你的数据库表对 name 和 slug 字段设置了 UNIQUE 约束，
        //    可以直接调用 repo.create，并在发生错误时处理数据库错误。
        //    更友好的方式是在 Service 层先检查。
        //    例如，如果 TagRepository 提供了 name_exists 和 slug_exists 方法：
        //    if self.repo.name_exists(name).await.context("检查标签名称是否存在失败")? {
        //        return Err(anyhow!("标签名称 '{}' 已存在", name));
        //    }
        //    if self.repo.slug_exists(&slug).await.context("检查 slug 是否存在失败")? {
        //        return Err(anyhow!("生成的 slug '{}' 已存在，请尝试修改标签名称", slug));
        //    }
        //    注意：上面的 name_exists 和 slug_exists 方法需要在 TagRepository trait 中定义并实现。
        //    目前我们的 TagRepository 还没有这些方法，所以暂时依赖数据库的唯一约束。

        // 4. 调用仓库创建标签
        let tag = self
            .repo
            .create(name, &slug)
            .await
            .context(format!("Service 未能创建标签 '{}'", name))?;
        // 如果依赖数据库 UNIQUE 约束，这里需要处理可能的数据库错误。
        // ApiError 层会尝试将特定的数据库唯一约束错误转换为用户友好的 anyhow 错误。

        Ok(tag)
    }

    /// 根据 ID 获取标签
    pub async fn get_tag_by_id(&self, id: Uuid) -> Result<Tag> {
        let tag_option = self
            .repo
            .get_by_id(id)
            .await
            .context(format!("Service 未能通过 ID ({}) 获取标签", id))?;

        // 将 Option<Tag> 转换为 Result<Tag, anyhow::Error>
        // 如果 tag_option 是 None，则返回一个错误
        tag_option.ok_or_else(|| anyhow!("未找到 ID 为 {} 的标签", id))
    }

    /// 根据 Slug 获取标签
    pub async fn get_tag_by_slug(&self, slug: &str) -> Result<Tag> {
        let tag_option = self
            .repo
            .get_by_slug(slug)
            .await
            .context(format!("Service 未能通过 slug ('{}') 获取标签", slug))?;

        tag_option.ok_or_else(|| anyhow!("未找到 Slug 为 '{}'", slug))
    }

    /// 获取所有标签列表
    pub async fn list_tags(&self) -> Result<Vec<Tag>> {
        self.repo.list().await.context("Service 未能获取标签列表")
    }

    /// 更新标签
    pub async fn update_tag(&self, id: Uuid, payload: UpdateTagPayload) -> Result<Tag> {
        let mut new_name_opt: Option<String> = None;
        let mut new_slug_opt: Option<String> = None;

        // 1. 验证并准备要更新的字段
        if let Some(name_to_update) = payload.name {
            let trimmed_name = name_to_update.trim();
            if trimmed_name.is_empty() {
                return Err(anyhow!("标签名不能为空"));
            }
            new_name_opt = Some(trimmed_name.to_string());
            new_slug_opt = Some(slugify(trimmed_name));

            // （可选但推荐）检查新的名称或 slug 是否会导致冲突 (排除当前正在更新的标签自身)
            // 例如:
            // let current_tag = self.get_tag_by_id(id).await?; // 获取当前标签用于比较
            // if let Some(ref nn) = new_name_opt {
            //     if nn != &current_tag.name && self.repo.name_exists(nn).await? { // 假设 name_exists 存在
            //         return Err(anyhow!("新的标签名称 '{}' 已被其他标签使用", nn));
            //     }
            // }
            // if let Some(ref ns) = new_slug_opt {
            //      if ns != &current_tag.slug && self.repo.slug_exists(ns).await? { // 假设 slug_exists 存在
            //          return Err(anyhow!("生成的新 slug '{}' 已被其他标签使用", ns));
            //      }
            // }
        }

        // 2. 如果没有任何要更新的字段（这里只考虑 name），可以直接返回当前标签或错误
        if new_name_opt.is_none() {
            return self
                .get_tag_by_id(id)
                .await
                .context("更新时未提供有效字段，尝试获取当前标签失败");
        }

        // 3. 调用仓库更新标签
        let tag = self
            .repo
            .update(id, new_name_opt.as_deref(), new_slug_opt.as_deref())
            .await
            .context(format!("Service 未能更新标签 (ID: {})", id))?;

        Ok(tag)
    }

    /// 删除标签
    pub async fn delete_tag(&self, id: Uuid) -> Result<()> {
        // 在删除前，可以添加业务逻辑检查，例如：
        // - 标签是否还关联着文章？如果有关联，是禁止删除、解除关联，还是其他操作？
        //   这取决于业务需求。目前，我们直接删除。
        self.repo
            .delete(id)
            .await
            .context(format!("Service 未能删除标签 (ID: {})", id))
    }

    // === 管理员专用功能 ===

    /// 合并标签：将多个标签合并为一个目标标签
    /// 所有被合并标签的文章关联将转移到目标标签
    pub async fn merge_tags(&self, target_tag_id: Uuid, source_tag_ids: &[Uuid]) -> Result<Tag> {
        // 1. 验证目标标签存在
        let target_tag = self
            .get_tag_by_id(target_tag_id)
            .await
            .context("合并操作：目标标签不存在")?;

        // 2. 验证源标签都存在且不包含目标标签
        for &source_id in source_tag_ids {
            if source_id == target_tag_id {
                return Err(anyhow!("不能将标签合并到自身"));
            }
            self.get_tag_by_id(source_id)
                .await
                .context(format!("合并操作：源标签 {} 不存在", source_id))?;
        }

        // 3. 调用仓库进行合并操作
        self.repo
            .merge_tags(target_tag_id, source_tag_ids)
            .await
            .context("标签合并操作失败")?;

        Ok(target_tag)
    }

    /// 增强版标签合并：提供详细的操作结果
    pub async fn merge_tags_enhanced(
        &self,
        target_tag_id: Uuid,
        source_tag_ids: &[Uuid],
        new_target_name: Option<&str>,
    ) -> Result<crate::dtos::tag::MergeTagsResponse> {
        // 1. 验证目标标签存在
        let mut target_tag = self
            .get_tag_by_id(target_tag_id)
            .await
            .context("合并操作：目标标签不存在")?;

        // 2. 验证源标签都存在且不包含目标标签
        let mut source_tags = Vec::new();
        for &source_id in source_tag_ids {
            if source_id == target_tag_id {
                return Err(anyhow!("不能将标签合并到自身"));
            }
            let source_tag = self
                .get_tag_by_id(source_id)
                .await
                .context(format!("合并操作：源标签 {} 不存在", source_id))?;
            source_tags.push(source_tag);
        }

        // 3. 执行增强合并操作
        let (affected_post_count, merged_tag_count, duplicate_relations_removed) = self
            .repo
            .merge_tags_enhanced(target_tag_id, source_tag_ids, new_target_name)
            .await
            .context("增强标签合并操作失败")?;

        // 4. 如果更新了目标标签名称，重新获取
        if new_target_name.is_some() {
            target_tag = self.get_tag_by_id(target_tag_id).await?;
        }

        // 5. 生成操作摘要
        let source_names: Vec<String> = source_tags.iter().map(|t| t.name.clone()).collect();
        let operation_summary = format!(
            "成功将 {} 个标签({}) 合并到 '{}'，影响了 {} 篇文章，移除了 {} 个重复关联",
            merged_tag_count,
            source_names.join(", "),
            target_tag.name,
            affected_post_count,
            duplicate_relations_removed
        );

        Ok(crate::dtos::tag::MergeTagsResponse {
            target_tag,
            merged_tag_count,
            affected_post_count,
            duplicate_relations_removed,
            operation_summary,
        })
    }

    /// 获取标签合并预览
    pub async fn get_merge_preview(
        &self,
        target_tag_id: Uuid,
        source_tag_ids: &[Uuid],
    ) -> Result<crate::dtos::tag::MergeTagsPreviewResponse> {
        // 1. 获取预览数据
        let (all_tags, total_posts_affected, posts_with_duplicates) = self
            .repo
            .get_merge_preview(target_tag_id, source_tag_ids)
            .await?;

        // 2. 分离目标标签和源标签
        let target_tag = all_tags
            .iter()
            .find(|t| t.id == target_tag_id)
            .ok_or_else(|| anyhow!("目标标签不存在"))?
            .clone();

        let source_tags: Vec<_> = all_tags
            .iter()
            .filter(|t| source_tag_ids.contains(&t.id))
            .cloned()
            .collect();

        // 3. 获取每个标签的文章统计
        let mut posts_by_tag = Vec::new();
        for tag in &all_tags {
            let (post_count, sample_titles) = self.repo.get_tag_post_info(tag.id).await?;
            posts_by_tag.push(crate::dtos::tag::TagPostCount {
                tag: tag.clone(),
                post_count,
                sample_post_titles: sample_titles,
            });
        }

        // 4. 生成潜在问题提醒
        let mut potential_issues = Vec::new();

        if posts_with_duplicates > 0 {
            potential_issues.push(format!(
                "有 {} 篇文章同时拥有目标标签和源标签，合并后将移除重复关联",
                posts_with_duplicates
            ));
        }

        if source_tags.is_empty() {
            potential_issues.push("没有找到有效的源标签".to_string());
        }

        let high_usage_tags: Vec<_> = posts_by_tag
            .iter()
            .filter(|tpc| tpc.post_count > 50)
            .map(|tpc| tpc.tag.name.clone())
            .collect();

        if !high_usage_tags.is_empty() {
            potential_issues.push(format!(
                "以下标签使用频率较高，请谨慎合并：{}",
                high_usage_tags.join(", ")
            ));
        }

        Ok(crate::dtos::tag::MergeTagsPreviewResponse {
            target_tag,
            source_tags,
            total_posts_affected,
            posts_with_duplicates,
            posts_by_tag,
            potential_issues,
        })
    }

    /// 批量删除标签
    pub async fn batch_delete_tags(&self, tag_ids: &[Uuid]) -> Result<usize> {
        let deleted_count = self
            .repo
            .batch_delete(tag_ids)
            .await
            .context("批量删除标签失败")?;

        Ok(deleted_count)
    }

    /// 获取标签使用统计（包含多少篇文章）
    pub async fn get_tag_usage_stats(&self) -> Result<Vec<TagUsageStats>> {
        self.repo
            .get_usage_stats()
            .await
            .context("获取标签使用统计失败")
    }

    /// 查找重复或相似的标签（用于清理建议）
    pub async fn find_similar_tags(&self) -> Result<Vec<SimilarTagGroup>> {
        self.repo
            .find_similar_tags()
            .await
            .context("查找相似标签失败")
    }
}

// 重新导出类型以保持API简洁
pub use crate::repositories::tag::{SimilarTagGroup, TagUsageStats};
