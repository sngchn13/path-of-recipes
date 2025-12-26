use uuid::Uuid;

use crate::domain::posts::{error::PostServiceError, presentation::NewPost};

mod implement;

pub use implement::PostServiceImpl;

#[async_trait::async_trait]
pub trait PostService: Send + Sync {
    async fn save_posts_with_related(&self, new_post: NewPost) -> Result<Uuid, PostServiceError>;
}