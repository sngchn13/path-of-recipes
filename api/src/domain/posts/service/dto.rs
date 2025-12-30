use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::{graphs::GraphAggregate, posts::Post};

#[derive(Debug)]
pub struct PostAggregate {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub view: i32,
    pub like: i32,
    pub dislike: i32,
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
            view: post.view, 
            like: post.like, 
            dislike: post.dislike, 
            graph, 
            created_at: post.created_at, 
            updated_at: post.updated_at 
        }
    }
}