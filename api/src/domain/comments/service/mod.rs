use crate::domain::comments::error::CommentError;

mod dto;
mod implement;

pub use implement::CommentServiceImpl;
pub use dto::CreateCommentInput;

#[async_trait::async_trait]
pub trait CommentService: Send + Sync {
    async fn save_comment(&self, new_comment: CreateCommentInput) -> Result<i32, CommentError>;
}