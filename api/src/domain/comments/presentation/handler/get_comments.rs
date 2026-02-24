use std::sync::Arc;
use axum::{Json, extract::{Path, Query, State}, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::{domain::comments::{CommentService, SearchCommentInput, error::CommentError, presentation::dto::CommentResponse}, shared::{Page, PageResponse, Pagination, PaginationQuery, SortDirection}};

#[axum::debug_handler]
pub async fn get_comments_by_conditon(
    State(comment_service): State<Arc<dyn CommentService>>,
    search_condition: Query<CommentSearchCondition>,
    Path(post_id): Path<Uuid>,
) ->  Result<(StatusCode, Json<PageResponse<CommentResponse>>), CommentError>

{
    let search_condition = search_condition.0;
    let condition = SearchCommentInput{ post_id, pagination: Pagination::from(search_condition.paging_query), order_by: search_condition.order_by.unwrap_or_default(), direction: search_condition.direction.unwrap_or(SortDirection::Asc) };

    let temp = comment_service.search_comments(condition).await?;
    let items: Vec<CommentResponse> = temp.content.into_iter().map(|c| CommentResponse::from(c)).collect();
    let pageable = temp.pageable;
    let paged_comments = Page::new(items, pageable);

    let response = PageResponse::from(paged_comments);
    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CommentSearchCondition {
    #[serde(flatten)]
    pub paging_query: PaginationQuery,
    pub order_by: Option<CommentOrder>,
    pub direction: Option<SortDirection>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CommentOrder {
    CreatedAt,
    LikeCount,
    ReplyCount,
}

impl Default for CommentOrder {
    fn default() -> Self {
        CommentOrder::CreatedAt
    }
}