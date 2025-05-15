use crate::models::Category;
use crate::repositories::CategoryRepository;
use anyhow::{Context, Result, anyhow};
use slug::slugify;
use std::sync::Arc;
use uuid::Uuid;
use crate::dtos::category::{CreateCategoryPayload, UpdateCategoryPayload};

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
}
