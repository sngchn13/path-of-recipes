use std::sync::Arc;
use axum::{Json, extract::{Path, Query, State}};
use serde::{Deserialize};
use tracing::debug;

use crate::{domain::comments::{CommentService, ReplyCommentCondition, error::CommentError, presentation::dto::CommentResponse}, shared::Cursor};


#[axum::debug_handler]
pub async fn get_replies_by_condition(
    State(comment_service): State<Arc<dyn CommentService>>,
    Query(reply_query): Query<ReplyQuery>,
    Path(comment_id): Path<i32>,
)   -> Result<Json<Cursor<CommentResponse>>, CommentError> 
{
    let condition = ReplyCommentCondition { parent_id: comment_id, after_id: reply_query.after_id, size: reply_query.size.unwrap_or(20) };
    
    let temp = comment_service.search_reply_comments(condition).await?;
    let items = temp.content;
    let next = temp.next_cursor;
    let items: Vec<CommentResponse> = items.into_iter().map(|c| CommentResponse::from(c)).collect();

    Ok(Json(Cursor::new(items, next)))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReplyQuery {
    pub size: Option<u64>,
    pub after_id: Option<i32>,
}