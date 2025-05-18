use crate::dtos::{CreateTagPayload, UpdateTagPayload};
use crate::models::Tag;
use crate::repositories::TagRepository;
use anyhow::{anyhow, Context, Result};
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
}
