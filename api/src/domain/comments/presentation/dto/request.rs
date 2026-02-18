use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewCommentRequest {
    pub author_name: Option<String>,
    pub password: String,
    pub parent_id: Option<i32>,
    pub content: String,
}