use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode};
use tracing::debug;
use uuid::Uuid;
use crate::domain::posts::{PatchPost, PatchedPostResponse, PostError, PostService, service::PatchPostCommand};

//patch /posts/{post_id}
#[axum::debug_handler]
pub async fn patch_post_by_id(
    State(post_service): State<Arc<dyn PostService>>,
    Path(post_id): Path<Uuid>,
    Json(patch_post): Json<PatchPost>
) -> Result<(StatusCode, Json<PatchedPostResponse>), PostError> 
{
    let command = PatchPostCommand { post_id, title: patch_post.title, password: patch_post.password, content: patch_post.content, graph: patch_post.graph };

    debug!("command = {:#?}", command);
    let post = post_service.update_post_with_related(command).await?;
    let res = PatchedPostResponse { post_id: post.id };

    Ok((StatusCode::OK, Json(res)))
}