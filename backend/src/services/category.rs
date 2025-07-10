use crate::dtos::category::{CreateCategoryPayload, UpdateCategoryPayload};
use crate::models::Category;
use crate::repositories::CategoryRepository;
use anyhow::{Context, Result, anyhow};
use slug::slugify;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)] // 需要Clone，以便在 AppState 中共享
pub struct CategoryService {
    repo: Arc<dyn CategoryRepository>, // 依赖 CategoryRepository
}

impl CategoryService {
    pub fn new(repo: Arc<dyn CategoryRepository>) -> Self {
        Self { repo }
    }

    /// 创建一个新的分类
    pub async fn create_category(&self, payload: CreateCategoryPayload) -> Result<Category> {
        // 1. 验证输入(例如：名称不能为空)
        let name_owned = payload.name.ok_or_else(|| anyhow!("分类名称不能为空"))?;
        let name = name_owned.trim();

        if name.is_empty() {
            return Err(anyhow!("分类名称不能为空"));
        }

        // 2. 生成 slug
        let slug = slugify(name);

        // 3. 检查名称或者slug是否已经存在
        //    这取决于你的业务规则。如果 name 和 slug 在数据库层面有 UNIQUE 约束，
        //    那么直接调用 repo.create 并在错误时处理数据库错误也是一种方式。
        //    更友好的方式是在 Service 层先检查。
        //    例如:
        //    if self.repo.name_exists(name).await.context("检查分类名称是否存在失败")? {
        //        return Err(anyhow!("分类名称 '{}' 已存在", name));
        //    }
        //    if self.repo.slug_exists(&slug).await.context("检查 slug 是否存在失败")? {
        //        // 如果 slug 冲突，可能需要一个更智能的 slug 生成策略（例如加后缀）
        //        // 这里为了简单，我们先假设 slugify(name) 足够独特，或依赖数据库约束
        //        return Err(anyhow!("生成的 slug '{}' 已存在，请尝试修改分类名称", slug));
        //    }
        //    注意：上面的 name_exists 和 slug_exists 方法需要在 CategoryRepository trait 中定义

        // 4. 调用仓库创建分类
        let category = self
            .repo
            .create(name, &slug)
            .await
            .context(format!("Service 未能创建分类 '{}'", name))?;
        // 如果依赖数据库 UNIQUE 约束，这里需要处理可能的数据库错误
        // 例如，将特定的数据库唯一约束错误转换为用户友好的 anyhow 错误
        Ok(category)
    }

    /// 根据 ID 获取分类
    pub async fn get_category_by_id(&self, id: Uuid) -> Result<Category> {
        let category_option = self
            .repo
            .get_by_id(id)
            .await
            .context(format!("Service 未能通过 ID ({}) 获取分类", id))?;
        category_option.ok_or_else(|| anyhow!("未找到 ID 为 {} 的分类", id))
    }

    /// 根据 Slug 获取分类
    pub async fn get_category_by_slug(&self, slug: &str) -> Result<Category> {
        let category_option = self
            .repo
            .get_by_slug(slug)
            .await
            .context(format!("Service 未能通过 slug ('{}') 获取分类", slug))?;

        category_option.ok_or_else(|| anyhow!("未找到 Slug 为 '{}' 的分类", slug))
    }

    /// 获取所有分类列表
    pub async fn list_categories(&self) -> Result<Vec<Category>> {
        self.repo.list().await.context("Service 未能获取分类列表")
    }

    /// 跟新分类
    pub async fn update_category(
        &self,
        id: Uuid,
        payload: UpdateCategoryPayload,
    ) -> Result<Category> {
        let mut new_name_opt: Option<String> = None;
        let mut new_slug_opt: Option<String> = None;

        // 1. 验证并准备要更新的字段段
        if let Some(name_to_update) = payload.name {
            let trimmed_name = name_to_update.trim();
            if trimmed_name.is_empty() {
                return Err(anyhow!("分类名称不能为空"));
            }
            new_name_opt = Some(trimmed_name.to_string());
            new_slug_opt = Some(slugify(trimmed_name));

            // (可选但推荐) 检查新的名称或 slug 是否会导致冲突 (排除当前正在更新的分类自身)
            // 例如:
            // let current_category = self.get_category_by_id(id).await?; // 获取当前分类用于比较
            // if let Some(ref nn) = new_name_opt {
            //     if nn != &current_category.name && self.repo.name_exists(nn).await? {
            //         return Err(anyhow!("新的分类名称 '{}' 已被其他分类使用", nn));
            //     }
            // }
            // if let Some(ref ns) = new_slug_opt {
            //      if ns != &current_category.slug && self.repo.slug_exists(ns).await? {
            //          return Err(anyhow!("生成的新 slug '{}' 已被其他分类使用", ns));
            //      }
            // }
        }
        // 2. 如果没有任何要更新的字段（这里只考虑 name），可以直接返回当前分类或错误
        if new_name_opt.is_none() {
            // 如果 UpdateCategoryPayload 允许更新其他字段，这里逻辑会更复杂
            // 目前它只有 name，所以如果 name 是 None，意味着没有有效更新
            // 可以选择返回错误，或者获取并返回未修改的分类
            return self
                .get_category_by_id(id)
                .await
                .context("更新时未提供有效字段，尝试获取当前分类失败");
            // return Err(anyhow!("没有提供要更新的分类信息"));
        }
        // 3. 调用仓库更新分类
        // 将 Option<String> 转换为 Option<&str> 给 repository
        let category = self
            .repo
            .update(id, new_name_opt.as_deref(), new_slug_opt.as_deref())
            .await
            .context(format!("Service 未能更新分类 (ID: {})", id))?;

        Ok(category)
    }

    /// 删除分类
    pub async fn delete_category(&self, id: Uuid) -> Result<()> {
        // 在删除前，可以添加业务逻辑检查，例如：
        // - 分类下是否还有文章？如果有关联文章，是禁止删除、将文章设为无分类，还是级联删除？
        //   (数据库层面我们设置了 ON DELETE CASCADE，但业务层面可能想做更友好的处理或提示)
        //   初期可以先直接删除。
        self.repo
            .delete(id)
            .await
            .context(format!("Service 未能删除分类 (ID：{})", id))
    }

    // === 管理员专用功能 ===

    /// 合并分类：将多个分类合并为一个目标分类
    /// 所有被合并分类的文章关联将转移到目标分类
    pub async fn merge_categories(
        &self,
        target_category_id: Uuid,
        source_category_ids: &[Uuid],
    ) -> Result<Category> {
        // 1. 验证目标分类存在
        let target_category = self
            .get_category_by_id(target_category_id)
            .await
            .context("合并操作：目标分类不存在")?;

        // 2. 验证源分类都存在且不包含目标分类
        for &source_id in source_category_ids {
            if source_id == target_category_id {
                return Err(anyhow!("不能将分类合并到自身"));
            }
            self.get_category_by_id(source_id)
                .await
                .context(format!("合并操作：源分类 {} 不存在", source_id))?;
        }

        // 3. 调用仓库进行合并操作
        self.repo
            .merge_categories(target_category_id, source_category_ids)
            .await
            .context("分类合并操作失败")?;

        Ok(target_category)
    }

    /// 增强版分类合并：提供详细的操作结果
    pub async fn merge_categories_enhanced(
        &self,
        target_category_id: Uuid,
        source_category_ids: &[Uuid],
        new_target_name: Option<&str>,
    ) -> Result<crate::dtos::category::MergeCategoriesResponse> {
        // 1. 验证目标分类存在
        let target_category = self
            .get_category_by_id(target_category_id)
            .await
            .context("合并操作：目标分类不存在")?;

        // 2. 验证源分类都存在且不包含目标分类
        let mut source_categories = Vec::new();
        for &source_id in source_category_ids {
            if source_id == target_category_id {
                return Err(anyhow!("不能将分类合并到自身"));
            }
            let source_category = self
                .get_category_by_id(source_id)
                .await
                .context(format!("合并操作：源分类 {} 不存在", source_id))?;
            source_categories.push(source_category);
        }

        // 3. 执行增强合并操作
        let (merged_count, affected_posts, duplicates_removed) = self
            .repo
            .merge_categories_enhanced(target_category_id, source_category_ids, new_target_name)
            .await
            .context("增强分类合并操作失败")?;

        // 4. 获取最新的目标分类信息（可能已更新名称）
        let final_target_category = self
            .get_category_by_id(target_category_id)
            .await
            .context("获取合并后的目标分类失败")?;

        // 5. 构建操作摘要
        let operation_summary = if let Some(new_name) = new_target_name {
            format!(
                "成功将 {} 个分类合并到 '{}' 分类，影响 {} 篇文章，移除 {} 个重复关联，并将目标分类重命名为 '{}'",
                merged_count, target_category.name, affected_posts, duplicates_removed, new_name
            )
        } else {
            format!(
                "成功将 {} 个分类合并到 '{}' 分类，影响 {} 篇文章，移除 {} 个重复关联",
                merged_count, final_target_category.name, affected_posts, duplicates_removed
            )
        };

        Ok(crate::dtos::category::MergeCategoriesResponse {
            target_category: final_target_category,
            merged_category_count: merged_count,
            affected_post_count: affected_posts,
            duplicate_relations_removed: duplicates_removed,
            operation_summary,
        })
    }

    /// 获取分类合并预览
    pub async fn get_merge_preview(
        &self,
        target_category_id: Uuid,
        source_category_ids: &[Uuid],
    ) -> Result<crate::dtos::category::MergeCategoriesPreviewResponse> {
        // 1. 验证目标分类存在
        let target_category = self
            .get_category_by_id(target_category_id)
            .await
            .context("预览操作：目标分类不存在")?;

        // 2. 验证源分类都存在且不包含目标分类
        let mut source_categories = Vec::new();
        for &source_id in source_category_ids {
            if source_id == target_category_id {
                return Err(anyhow!("不能将分类合并到自身"));
            }
            let source_category = self
                .get_category_by_id(source_id)
                .await
                .context(format!("预览操作：源分类 {} 不存在", source_id))?;
            source_categories.push(source_category);
        }

        // 3. 获取预览信息
        let (all_categories, total_affected, duplicates) = self
            .repo
            .get_merge_preview(target_category_id, source_category_ids)
            .await
            .context("获取分类合并预览失败")?;

        // 4. 构建每个分类的详细信息
        let mut posts_by_category = Vec::new();
        for category in &all_categories {
            let (post_count, sample_titles) =
                self.repo
                    .get_category_post_info(category.id)
                    .await
                    .context(format!("获取分类 {} 的文章信息失败", category.id))?;

            posts_by_category.push(crate::dtos::category::CategoryPostCount {
                category: category.clone(),
                post_count,
                sample_post_titles: sample_titles,
            });
        }

        // 5. 生成潜在问题提醒
        let mut potential_issues = Vec::new();

        // 检查高频分类
        for category_info in &posts_by_category {
            if category_info.post_count > 50
                && source_category_ids.contains(&category_info.category.id)
            {
                potential_issues.push(format!(
                    "⚠️ 分类 '{}' 有 {} 篇文章，删除后这些文章将转移到目标分类",
                    category_info.category.name, category_info.post_count
                ));
            }
        }

        if duplicates > 0 {
            potential_issues.push(format!(
                "ℹ️ 有 {} 篇文章同时使用了源分类和目标分类，将自动去重",
                duplicates
            ));
        }

        if potential_issues.is_empty() {
            potential_issues.push("✅ 未发现潜在问题，可以安全执行合并操作".to_string());
        }

        Ok(crate::dtos::category::MergeCategoriesPreviewResponse {
            target_category,
            source_categories,
            total_posts_affected: total_affected,
            posts_with_duplicates: duplicates,
            posts_by_category,
            potential_issues,
        })
    }

    /// 批量删除分类
    pub async fn batch_delete_categories(&self, category_ids: &[Uuid]) -> Result<usize> {
        // 验证所有分类都存在
        for &category_id in category_ids {
            self.get_category_by_id(category_id)
                .await
                .context(format!("批量删除：分类 {} 不存在", category_id))?;
        }

        self.repo
            .batch_delete(category_ids)
            .await
            .context("批量删除分类操作失败")
    }

    /// 增强版批量删除分类（支持孤儿文章处理）
    pub async fn batch_delete_categories_enhanced(
        &self,
        category_ids: &[Uuid],
        handle_orphaned: bool,
    ) -> Result<crate::dtos::category::BatchDeleteCategoriesResponse> {
        // 验证所有分类都存在
        for &category_id in category_ids {
            self.get_category_by_id(category_id)
                .await
                .context(format!("批量删除：分类 {} 不存在", category_id))?;
        }

        let (deleted_count, affected_post_count, orphaned_post_count) = self
            .repo
            .batch_delete_enhanced(category_ids, handle_orphaned)
            .await
            .context("增强版批量删除分类操作失败")?;

        let operation_summary = if handle_orphaned && orphaned_post_count > 0 {
            format!(
                "成功删除 {} 个分类，影响了 {} 篇文章，其中 {} 篇孤儿文章已自动归类到'未分类'",
                deleted_count, affected_post_count, orphaned_post_count
            )
        } else {
            format!(
                "成功删除 {} 个分类，影响了 {} 篇文章",
                deleted_count, affected_post_count
            )
        };

        Ok(crate::dtos::category::BatchDeleteCategoriesResponse {
            deleted_count,
            affected_post_count,
            orphaned_post_count,
            operation_summary,
        })
    }

    /// 获取分类使用统计
    pub async fn get_category_usage_stats(
        &self,
    ) -> Result<Vec<crate::repositories::category::CategoryUsageStats>> {
        self.repo
            .get_usage_stats()
            .await
            .context("获取分类使用统计失败")
    }

    /// 查找相似分类
    pub async fn find_similar_categories(
        &self,
    ) -> Result<Vec<crate::repositories::category::SimilarCategoryGroup>> {
        self.repo
            .find_similar_categories()
            .await
            .context("查找相似分类失败")
    }
}
