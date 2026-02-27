use std::collections::HashSet;

use uuid::Uuid;

use crate::{domain::{nodes::NodeAggregate, posts::{Post, error::PostError, presentation::{NewPost, PostOrder}}}, shared::{Page, Pagination, SortDirection}};

mod implement;
mod dto;

pub use implement::PostServiceImpl;
pub use dto::*;

#[async_trait::async_trait]
pub trait PostService: Send + Sync {
    async fn save_posts_with_related(&self, new_post: NewPost) -> Result<Uuid, PostError>;
    async fn find_post_with_related(&self, post_id: Uuid) -> Result<PostAggregate, PostError>;
    async fn find_posts_with_representative_node(&self, mod_group_ids: HashSet<i32>, pagination: Pagination, order_by: PostOrder,direction: SortDirection) -> Result<Page<(Post, NodeAggregate)>, PostError>;
    async fn update_post_with_related(&self, patch_post_command: PatchPostCommand) -> Result<Post, PostError>;
}