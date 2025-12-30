use uuid::Uuid;

use crate::domain::posts::{error::PostServiceError, presentation::NewPost};

mod implement;
mod dto;

pub use implement::PostServiceImpl;
pub use dto::*;

#[async_trait::async_trait]
pub trait PostService: Send + Sync {
    async fn save_posts_with_related(&self, new_post: NewPost) -> Result<Uuid, PostServiceError>;
    async fn find_post_with_related(&self, post_id: Uuid) -> Result<PostAggregate, PostServiceError>;
}