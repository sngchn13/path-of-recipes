use std::{collections::HashSet, sync::Arc};
use axum::{Json, extract::{self, State}, http::StatusCode};
use axum_extra::extract::Query;
use serde::Deserialize;
use tracing::{info};
use uuid::Uuid;

use crate::{domain::posts::{PostService, PostSummary, error::PostHandlerError, presentation::{CreatePostResponse, PostResponse, dto::NewPost}}, shared::{Page, PageResponse, Pagination, PaginationQuery, SortDirection}};


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
    let post = PostResponse::from(post_service.find_post_with_related(post_id).await?);

    Ok((StatusCode::OK, Json(post)))
}

#[axum::debug_handler]
pub async fn get_posts_by_condition(
    State(post_service): State<Arc<dyn PostService>>,
    Query(condition): Query<PostSearchCondition>
) -> Result<(StatusCode, Json<PageResponse<PostSummary>>), PostHandlerError> 
{
    let msg = format!("condition = {:#?}", condition);
    info!(msg);
    let page_posts = post_service.find_posts_with_representative_node(condition.mod_group_ids, Pagination::from(condition.paging_query),
        condition.order_by.unwrap_or_default(),condition.direction.unwrap_or_default()).await?
        .map(|(post, node)| PostSummary::from(post, node));

    let response = PageResponse::from(page_posts);

    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostSearchCondition {
    #[serde(flatten)]
    pub paging_query: PaginationQuery,
    
    #[serde(rename = "order-by")]
    pub order_by: Option<PostOrder>,

    pub direction: Option<SortDirection>,
    #[serde(default)]
    pub mod_group_ids: HashSet<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PostOrder {
    View,
    Like,
    CreatedAt,
}

impl Default for PostOrder {
    fn default() -> Self {
        PostOrder::CreatedAt
    }
}