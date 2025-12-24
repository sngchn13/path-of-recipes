use axum::{extract, http::StatusCode, response::IntoResponse};
use tracing::info;

use crate::domain::posts::presentation::dto::NewPost;


#[axum::debug_handler]
pub async fn post_new_post(extract::Json(new_post): extract::Json<NewPost>) -> impl IntoResponse{
    let tmp = format!("new post is {:#?}", new_post);
    info!(tmp);

    StatusCode::CREATED
}