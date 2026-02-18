use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::comments::entity::{Comment, Status};


#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: i32,
    pub post_id: Uuid,
    pub author_name: Option<String>,
    pub parent_id: Option<i32>,
    pub content: Option<String>,
    pub status: Status,
    pub like_count: i32,
    pub dislike_count: i32,
    pub reply_count: i32,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
}

impl From<Comment> for CommentResponse {
    fn from(value: Comment) -> Self {
        let content = match value.status {
            Status::Active => Some(value.content),
            _ => None,
        };

        Self { id: value.id, post_id: value.post_id, author_name: value.author_name, parent_id: value.parent_id, content: content, status: value.status, like_count: value.like_count, dislike_count: value.dislike_count, reply_count: value.reply_count, created_at: value.created_at, updated_at: value.updated_at }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateCommentResponse{
    pub id: i32
}