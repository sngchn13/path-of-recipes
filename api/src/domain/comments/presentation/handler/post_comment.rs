use std::sync::Arc;
use axum::{Json, extract::{self, Path, State}, http::StatusCode};
use uuid::Uuid;

use crate::{app::AnonId, domain::comments::{CommentService, CreateCommentInput, error::CommentError, presentation::dto::{CreateCommentResponse, NewCommentRequest}}};


#[axum::debug_handler]
pub async fn post_new_comment(
    State(comment_service): State<Arc<dyn CommentService>>,
    anon_id: AnonId,
    Path(post_id): Path<Uuid>,
    extract::Json(new_comment): extract::Json<NewCommentRequest>
) -> Result<(StatusCode, Json<CreateCommentResponse>), CommentError> {
    let comment_input = CreateCommentInput::new(post_id, anon_id.0, new_comment.author_name, new_comment.password, new_comment.parent_id, new_comment.content);

    let saved_comment_id = comment_service.save_comment(comment_input).await?;

    let res = CreateCommentResponse{ id: saved_comment_id };
    Ok((StatusCode::CREATED, Json(res)))
}