use std::sync::Arc;
use axum::{extract::{self, State}};

use crate::domain::posts::{PostService, error::PostHandlerError, presentation::{CreatePostResponse, dto::NewPost}};


#[axum::debug_handler]
pub async fn post_new_post(
    State(post_service): State<Arc<dyn PostService>>,
    extract::Json(new_post): extract::Json<NewPost>) 
    -> Result<CreatePostResponse, PostHandlerError>
{
    let post_id = post_service.save_posts_with_related(new_post).await?;
    Ok(CreatePostResponse::new(post_id))
}