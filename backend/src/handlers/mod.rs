pub mod admin;
pub mod auth;
pub mod category;
pub mod post;
pub mod tag;
pub mod user;

pub use post::{
    create_post_handler, delete_post_handler, get_post_handler, list_posts_handler,
    update_post_handler,
};

pub use category::{
    create_category_handler, delete_category_handler, get_category_handler,
    list_categories_handler, update_category_handler,
};

pub use tag::{
    create_tag_handler, delete_tag_handler, get_tag_handler, list_tags_handler, update_tag_handler,
};

pub use admin::{create_permission_handler, create_role_handler, delete_permission_handler, delete_role_handler, list_permissions_handler, list_roles_handler, list_users_handler, set_role_permissions_handler, set_user_roles_handler, update_permission_handler, update_role_handler};

pub use auth::{login_handler, register_handler};

pub use user::{change_my_password_handler, delete_my_account_handler, get_my_profile_handler, update_my_profile_handler};

use crate::services::{AdminService, AuthService, CategoryService, PostService, TagService, UserService};

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
