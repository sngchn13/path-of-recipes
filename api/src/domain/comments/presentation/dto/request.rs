use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCommentRequest {
    pub author_name: Option<String>,
    pub password: String,
    pub parent_id: Option<i32>,
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchCommentRequest {
    pub password: String,
    pub content: String,
}