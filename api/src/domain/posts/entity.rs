use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct Post{
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub password: String,
    pub view_count: i32,
    pub like_count: i32,
    pub dislike_count: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl Post {
    pub fn new(id: Uuid, title: String, content: String, password: String, view: i32, like: i32, dislike: i32, created_at: OffsetDateTime, updated_at: Option<OffsetDateTime>) -> Self {
        Self { id, title, content, password, view_count: view, like_count: like, dislike_count: dislike, created_at, updated_at }
    }
}

