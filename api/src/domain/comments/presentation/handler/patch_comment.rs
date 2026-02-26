use std::sync::Arc;
use axum::{Json, extract::{Path, State}, http::StatusCode};

use crate::domain::comments::{CommentService, PatchCommentCommand, error::CommentError, presentation::dto::{CommentResponse, PatchCommentRequest}};


// patch /comments/{comment_id}
#[axum::debug_handler]
pub async fn patch_comment_by_id(
    State(comment_service): State<Arc<dyn CommentService>>,
    Path(comment_id): Path<i32>,
    Json(patch_comment): Json<PatchCommentRequest>
)   -> Result<(StatusCode, Json<CommentResponse>), CommentError> 
{
    let patch_comment_command = PatchCommentCommand{ comment_id, password: patch_comment.password, content: patch_comment.content };
    let res = CommentResponse::from(comment_service.patch_comment(patch_comment_command).await?);

    Ok((StatusCode::OK, Json(res)))
}