use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::{graphs::GraphAggregate, posts::Post};

#[derive(Debug)]
pub struct PostAggregate {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub view_count: i32,
    pub like_count: i32,
    pub dislike_count: i32,
    pub graph: GraphAggregate,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl PostAggregate {
    pub fn assemble_post(post: Post, graph: GraphAggregate) -> Self {
        Self { 
            id: post.id, 
            title: post.title, 
            content: post.content, 
            view_count: post.view_count, 
            like_count: post.like_count, 
            dislike_count: post.dislike_count, 
            graph, 
            created_at: post.created_at, 
            updated_at: post.updated_at 
        }
    }
}