use std::fmt::Display;

use entity::sea_orm_active_enums;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::shared::{fixed_offset_opt_to_offset_opt, fixed_offset_to_offset_datetime};

pub struct Comment {
    pub id: i32,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub author_name: Option<String>,
    pub password: String,
    pub parent_id: Option<i32>,
    pub content: String,
    pub status: Status,
    pub like_count: i32,
    pub dislike_count: i32,
    pub reply_count: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Active,
    Deleted,
    Hidden,
}

impl From<sea_orm_active_enums::Status> for Status {
    fn from(value: sea_orm_active_enums::Status) -> Self {
        match value {
            sea_orm_active_enums::Status::Active => Self::Active,
            sea_orm_active_enums::Status::Deleted => Self::Deleted,
            sea_orm_active_enums::Status::Hidden => Self::Hidden,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Status::Active => "active",
            Status::Deleted => "deleted",
            Status::Hidden => "hidden",
        };

        write!(f, "{}", s)
    }
}

impl From<entity::comments::Model> for Comment {
    fn from(value: entity::comments::Model) -> Self {
        Self { 
            id: value.id, 
            post_id: value.post_id, 
            author_id: value.author_id, 
            author_name: value.author_name, 
            password: value.password, 
            parent_id: value.parent_id, 
            content: value.content, 
            status: Status::from(value.status), 
            like_count: value.like_count, 
            dislike_count: value.dislike_count, 
            reply_count: value.reply_count, 
            created_at: fixed_offset_to_offset_datetime(value.created_at), 
            updated_at: fixed_offset_opt_to_offset_opt(value.updated_at), 
        }
    }
}