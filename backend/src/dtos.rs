use serde::{Deserialize, Serialize};

// 用于接收分页查询参数的结构体
#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    page: u64, // 页码（从1开始）
    #[serde(default = "default_page_size")]
    page_size: u64, // 每页数量
}

// page 的默认值函数
fn default_page() -> u64 {
    1 // 默认页码为 1
}

// page_size 的默认值函数
fn default_page_size() -> u64 {
    10 // 默认每页 10 条
}

impl Pagination {
    // 获取经过验证和调整的页码
    pub fn page(&self) -> u64 {
        // 页码至少为1
        if self.page == 0 { 1 } else { self.page }
    }

    // 获取经过验证和调整的每页数量
    pub fn page_size(&self) -> u64 {
        // 可以设置一个最大值，防止一次请求过多数据
        const MAX_PAGE_SIZE: u64 = 100;
        if self.page_size == 0 {
            default_page_size()
        } else if self.page_size > MAX_PAGE_SIZE {
            MAX_PAGE_SIZE
        } else {
            self.page_size
        }
    }

    // 计算数据库查询所需的 limit
    pub fn limit(&self) -> i64 {
        self.page_size() as i64 // SQL LIMIT 通常接收 i64 的值
    }

    // 计算数据库查询所需的 offset
    pub fn offset(&self) -> i64 {
        // SQL OFFSET 通常接受 i64
        // 页码从 1 开始，所以 offset = (page-1) * page_size
        ((self.page() - 1) * self.page_size()) as i64
    }
}

// 通用的分页响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,    // 当前页的数据列表
    pub total_items: i64, // 总记录数（从数据库 COUNT(*) 获取）
    pub page: u64,        // 当前页码
    pub page_size: u64,   // 每页数量
    pub total_pages: i64, // 总页数(计算得出）
}

impl<T> PaginatedResponse<T> {
    // 构造函数，自动计算 total_pages
    pub fn new(items: Vec<T>, total_items: i64, page: u64, page_size: u64) -> Self {
        // 计算总页数
        let total_pages = if page_size == 0 {
            // 防止除 0
            0
        } else {
            // (total_items + page_size - 1) / page_size 一种常用的向上取整技巧
            // (total_items as u64 + page_size - 1) / page_size
            (total_items as u64).div_ceil(page_size)
        } as i64;
        PaginatedResponse {
            items,
            total_items,
            page,
            page_size,
            total_pages,
        }
    }
}

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
