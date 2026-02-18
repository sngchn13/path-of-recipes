use crate::{domain::comments::{entity::Comment, error::CommentError}, shared::Page};

mod dto;
mod implement;

pub use implement::CommentServiceImpl;
pub use dto::{CreateCommentInput, SearchCommentInput};

#[async_trait::async_trait]
pub trait CommentService: Send + Sync {
    async fn save_comment(&self, new_comment: CreateCommentInput) -> Result<i32, CommentError>;
    async fn search_comments(&self, condition: SearchCommentInput) -> Result<Page<Comment>, CommentError>;
}