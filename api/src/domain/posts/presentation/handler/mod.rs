use std::sync::Arc;
use axum::{Json, extract::{self, State}, http::StatusCode};
use tracing::{info};
use uuid::Uuid;

use crate::domain::posts::{PostService, error::PostHandlerError, presentation::{CreatePostResponse, PostResponse, dto::NewPost}};


#[axum::debug_handler]
pub async fn post_new_post(
    State(post_service): State<Arc<dyn PostService>>,
    extract::Json(new_post): extract::Json<NewPost>) 
    -> Result<CreatePostResponse, PostHandlerError>
{
    let post_id = post_service.save_posts_with_related(new_post).await?;
    Ok(CreatePostResponse::new(post_id))
}

#[axum::debug_handler]
pub async fn get_post_by_id(
    State(post_service): State<Arc<dyn PostService>>,
    extract::Path(post_id): extract::Path<Uuid>
) -> Result<(StatusCode, Json<PostResponse>), PostHandlerError>
{
    let msg = format!("requested id = {}", post_id);
    info!(msg);
    let post = PostResponse::from(post_service.find_post_with_related(post_id).await?);

    Ok((StatusCode::OK, Json(post)))
}