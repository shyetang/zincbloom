pub mod category;
pub mod post;

pub use post::{
    create_post_handler, delete_post_handler, get_post_handler, list_posts_handler,
    update_post_handler,
};

pub use category::{
    create_category_handler, delete_category_handler, get_category_handler,
    list_categories_handler, update_category_handler,
};

use crate::services::{CategoryService, PostService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub post_service: Arc<PostService>,
    pub category_service: Arc<CategoryService>,
}
