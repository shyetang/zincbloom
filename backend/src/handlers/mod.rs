pub mod admin;
pub mod auth;
pub mod category;
pub mod post;
pub mod tag;

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

pub use auth::{login_handler, register_handler};

use crate::services::{AdminService, AuthService, CategoryService, PostService, TagService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub post_service: Arc<PostService>,
    pub category_service: Arc<CategoryService>,
    pub tag_service: Arc<TagService>,
    pub auth_service: Arc<AuthService>,
    pub admin_service: Arc<AdminService>,
}
