use crate::handlers::admin::{
    create_permission_handler, create_role_handler, create_user_handler, delete_permission_handler,
    delete_role_handler, delete_user_handler, get_dashboard_stats_handler, get_user_handler,
    get_user_stats_handler, list_permissions_handler, list_roles_handler,
    list_roles_with_permissions_handler, list_users_handler, reset_user_password_handler,
    set_role_permissions_handler, set_user_roles_handler, update_permission_handler,
    update_role_handler, update_user_handler,
};
use crate::handlers::auth::{
    forgot_password_handler, login_handler, logout_handler, refresh_token_handler,
    register_handler, reset_password_handler, verify_email_handler,
};
use crate::handlers::category::{
    create_category_handler, delete_category_handler, get_category_handler,
    list_categories_handler, update_category_handler,
};
use crate::handlers::post::{
    create_post_handler, delete_post_handler, get_post_handler, get_published_post_handler,
    list_posts_handler, list_published_posts_handler, publish_post_handler, unpublish_post_handler,
    update_post_handler,
};
use crate::handlers::tag::{
    create_tag_handler, delete_tag_handler, get_tag_handler, list_tags_handler, update_tag_handler,
};
use crate::handlers::user::{
    change_my_password_handler, delete_my_account_handler, get_my_profile_handler,
    get_my_stats_handler, update_my_profile_handler,
};
use crate::handlers::{
    AppState, batch_delete_tags_handler, find_similar_tags_handler, get_merge_preview_handler,
    get_tag_usage_stats_handler, list_user_permissions_handler, merge_tags_enhanced_handler,
    merge_tags_handler,
};
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // --- 健康检查端点 ---
        .route("/health", get(|| async { "OK" }))
        // --- 当前用户("我")的路由 ---
        .route(
            "/me",
            get(get_my_profile_handler).put(update_my_profile_handler),
        )
        .route("/me", delete(delete_my_account_handler))
        .route("/me/password", put(change_my_password_handler))
        .route("/me/permissions", get(list_user_permissions_handler))
        .route("/me/stats", get(get_my_stats_handler))
        // --- Admin 相关的路由 ---
        .route("/admin/stats/dashboard", get(get_dashboard_stats_handler))
        .route("/admin/stats/users", get(get_user_stats_handler))
        .route("/admin/users/{id}/roles", put(set_user_roles_handler))
        .route(
            "/admin/roles/{role_id}/permissions",
            put(set_role_permissions_handler),
        )
        .route(
            "/admin/users",
            get(list_users_handler).post(create_user_handler),
        )
        .route(
            "/admin/users/{user_id}",
            get(get_user_handler)
                .put(update_user_handler)
                .delete(delete_user_handler),
        )
        .route(
            "/admin/users/{user_id}/password",
            put(reset_user_password_handler),
        )
        .route(
            "/admin/roles",
            get(list_roles_handler).post(create_role_handler),
        )
        .route(
            "/admin/roles/with-permissions",
            get(list_roles_with_permissions_handler),
        )
        .route(
            "/admin/roles/{role_id}",
            put(update_role_handler).delete(delete_role_handler),
        )
        .route(
            "/admin/permissions",
            get(list_permissions_handler).post(create_permission_handler),
        )
        .route(
            "/admin/permissions/{permission_id}",
            put(update_permission_handler).delete(delete_permission_handler),
        )
        // -- Auth 相关路由 --
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/refresh", post(refresh_token_handler))
        .route("/auth/logout", post(logout_handler))
        .route("/auth/verify-email", post(verify_email_handler))
        .route("/auth/forgot-password", post(forgot_password_handler))
        .route("/auth/reset-password", post(reset_password_handler))
        // -- 博客公开接口（无需认证）--
        .route("/blog/posts", get(list_published_posts_handler))
        .route("/blog/posts/{identifier}", get(get_published_post_handler))
        // -- Post 管理接口（需要认证）--
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
        // 文章发布和撤回路由
        .route("/posts/{id}/publish", put(publish_post_handler))
        .route("/posts/{id}/unpublish", put(unpublish_post_handler))
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
        // --- 标签管理路由（管理员专用）---
        .route("/admin/tags/merge", post(merge_tags_handler))
        .route("/admin/tags/batch-delete", post(batch_delete_tags_handler))
        .route("/admin/tags/usage-stats", get(get_tag_usage_stats_handler))
        .route("/admin/tags/similar", get(find_similar_tags_handler))
        .route(
            "/admin/tags/merge-enhanced",
            post(merge_tags_enhanced_handler),
        )
        .route("/admin/tags/merge-preview", post(get_merge_preview_handler))
        .with_state(app_state) // 将共享状态注入路由
}
