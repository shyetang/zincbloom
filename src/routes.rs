use crate::handlers::{
    AppState, create_post_handler, delete_post_handler, get_post_handler, list_posts_handler,
    update_post_handler,
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
            "/posts/{id}",
            get(get_post_handler)
                .put(update_post_handler)
                .delete(delete_post_handler),
        )
        .with_state(app_state)  // 将共享状态注入路由
}
