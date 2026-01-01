use std::collections::HashSet;

use uuid::Uuid;

use crate::{domain::{nodes::NodeAggregate, posts::{Post, error::PostServiceError, presentation::{NewPost, PostOrder}}}, shared::{Page, Pagination, SortDirection}};

mod implement;
mod dto;

pub use implement::PostServiceImpl;
pub use dto::*;

#[async_trait::async_trait]
pub trait PostService: Send + Sync {
    async fn save_posts_with_related(&self, new_post: NewPost) -> Result<Uuid, PostServiceError>;
    async fn find_post_with_related(&self, post_id: Uuid) -> Result<PostAggregate, PostServiceError>;
    async fn find_posts_with_representative_node(&self, mod_group_ids: HashSet<i32>, pagination: Pagination, order_by: PostOrder,direction: SortDirection) -> Result<Page<(Post, NodeAggregate)>, PostServiceError>;
}