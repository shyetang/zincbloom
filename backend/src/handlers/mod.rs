pub mod admin;
pub mod auth;
pub mod category;
pub mod post;
pub mod tag;
pub mod user;

pub use post::{
    create_post_handler, delete_post_handler, get_post_handler, get_published_post_handler,
    list_posts_handler, list_published_posts_handler, update_post_handler,
};

pub use category::{
    create_category_handler, delete_category_handler, get_category_handler,
    list_categories_handler, update_category_handler,
};

pub use tag::{
    batch_delete_tags_handler, create_tag_handler, delete_tag_handler, find_similar_tags_handler,
    get_merge_preview_handler, get_tag_handler, get_tag_usage_stats_handler, list_tags_handler,
    merge_tags_enhanced_handler, merge_tags_handler, update_tag_handler,
};

pub use admin::{
    create_permission_handler, create_role_handler, create_user_handler, delete_permission_handler,
    delete_role_handler, delete_user_handler, get_dashboard_stats_handler, get_user_handler,
    get_user_stats_handler, list_permissions_handler, list_roles_handler,
    list_roles_with_permissions_handler, list_user_permissions_handler, list_users_handler,
    reset_user_password_handler, set_role_permissions_handler, set_user_roles_handler,
    update_permission_handler, update_role_handler, update_user_handler,
};

pub use auth::{
    forgot_password_handler, login_handler, refresh_token_handler, register_handler,
    reset_password_handler, verify_email_handler,
};

pub use user::{
    change_my_password_handler, delete_my_account_handler, get_my_profile_handler,
    update_my_profile_handler,
};

use crate::services::{
    AdminService, AuthService, CategoryService, PostService, TagService, UserService,
};

use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub post_service: Arc<PostService>,
    pub category_service: Arc<CategoryService>,
    pub tag_service: Arc<TagService>,
    pub auth_service: Arc<AuthService>,
    pub admin_service: Arc<AdminService>,
    pub user_service: Arc<UserService>,
}
