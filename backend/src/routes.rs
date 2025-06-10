use crate::handlers::{
    create_category_handler, create_post_handler, create_tag_handler, delete_category_handler,
    delete_post_handler, delete_tag_handler, get_category_handler, get_post_handler,
    get_tag_handler, list_categories_handler, list_posts_handler, list_tags_handler,
    login_handler, register_handler, update_category_handler, update_post_handler,
    update_tag_handler, AppState,
};
use axum::routing::{get, post, put};
use axum::Router;
use crate::handlers::admin::set_user_roles_handler;
use crate::handlers::auth::logout_handler;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // --- Admin 相关的路由 ---
        .route("/admin/users/{id}/roles",put(set_user_roles_handler))
        // -- Auth 相关路由 --
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/refresh",post(register_handler))
        .route("/auth/logout",post(logout_handler))
        // -- Post 相关路由 --
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
        // --- Tag 相关的路由 ---
        // GET /tags -> 获取标签列表
        // POST /tags -> 创建新标签
        .route("/tags", get(list_tags_handler).post(create_tag_handler))
        // GET /tags/{identifier} -> 获取单个标签详情 (identifier 可以是 id 或 slug)
        // PUT /tags/{id} -> 更新标签 (通常通过 id 更新)
        // DELETE /tags/{id} -> 删除标签 (通常通过 id 删除)
        .route(
            "/tags/{identifier}",
            get(get_tag_handler)
                .put(update_tag_handler)
                .delete(delete_tag_handler),
        )
        .with_state(app_state) // 将共享状态注入路由
}
