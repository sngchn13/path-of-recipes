use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct Post{
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub password: String,
    pub view: i32,
    pub like: i32,
    pub dislike: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl Post {
    pub fn new(id: Uuid, title: String, content: String, password: String, view: i32, like: i32, dislike: i32, created_at: OffsetDateTime, updated_at: Option<OffsetDateTime>) -> Self {
        Self { id, title, content, password, view, like, dislike, created_at, updated_at }
    }
}

