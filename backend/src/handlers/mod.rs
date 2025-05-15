pub mod post;

pub use post::{
    create_post_handler, delete_post_handler, get_post_handler, list_posts_handler,
    update_post_handler,
};

use std::sync::Arc;

use crate::services::{CategoryService,PostService};

#[derive(Clone)]
pub struct AppState {
    pub post_service: Arc<PostService>,
    pub category_service: Arc<CategoryService>,
}