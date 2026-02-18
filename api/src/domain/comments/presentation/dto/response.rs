use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::comments::entity::Status;


#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: i32,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub author_name: Option<String>,
    pub parent_id: Option<i32>,
    pub content: String,
    pub status: Status,
    pub like_count: i32,
    pub dislike_count: i32,
    pub reply_count: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize)]
pub struct CreateCommentResponse{
    pub id: i32
}