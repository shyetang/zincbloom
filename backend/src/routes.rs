use crate::handlers::{
    AppState, create_category_handler, create_post_handler, delete_category_handler,
    delete_post_handler, get_category_handler, get_post_handler, list_categories_handler,
    list_posts_handler, update_category_handler, update_post_handler,
};
use axum::Router;
use axum::routing::{get, post};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // 路由: /posts (复数)
        // POST /posts -> 创建帖子
        // GET  /posts -> 获取帖子列表
        .route("/posts", post(create_post_handler).get(list_posts_handler))
        // 路由: /posts/{id} (复数，使用花括号捕获 id)
        // GET    /posts/{id} -> 获取单个帖子
        // PUT    /posts/{id} -> 更新单个帖子
        // DELETE /posts/{id} -> 删除单个帖子
        .route(
            "/posts/{identifier}", // 使用通用占位符名 "identifier"
            get(get_post_handler)
                .put(update_post_handler)
                .delete(delete_post_handler),
        )
        // --- Category 相关的路由 ---
        // GET /categories -> 获取列表,
        // POST /categories -> 创建分类
        .route(
            "/categories",
            get(list_categories_handler).post(create_category_handler),
        )
        // GET /categories/{id_or_slug} -> 获取单个分类详情
        // PUT /categories/{id} -> 更新,
        // DELETE /categories/{id} -> 删除
        .route(
            "/categories/{identifier}", // 使用通用占位符名 "identifier"
            get(get_category_handler)
                .put(update_category_handler)
                .delete(delete_category_handler),
        )
        .with_state(app_state) // 将共享状态注入路由
}
