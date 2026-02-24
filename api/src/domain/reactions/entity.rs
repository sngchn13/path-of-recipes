use std::{fmt::Display, num::ParseIntError, str::FromStr};

use entity::reactions;
use serde::Deserialize;
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{shared::{fixed_offset_opt_to_offset_opt, fixed_offset_to_offset_datetime}};

#[derive(Debug)]
pub struct Reaction {
    pub id: i32,
    pub subject: Subject,
    pub anon_id: Uuid,
    pub reaction_type: ReactionType,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl TryFrom<reactions::Model> for Reaction {
    type Error = ReactionParseError;

    fn try_from(value: reactions::Model) -> Result<Self, Self::Error> {
        let subject = Subject::from_str(&value.subject)?;
        let reaction = Self { id: value.id, subject, anon_id: value.anon_id, reaction_type: ReactionType::from(value.reaction_type), created_at: fixed_offset_to_offset_datetime(value.created_at), updated_at: fixed_offset_opt_to_offset_opt(value.updated_at) };

        Ok(reaction)
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ReactionType {
    Like,
    Dislike,
}

impl From<entity::sea_orm_active_enums::ReactionType> for ReactionType {
    fn from(value: entity::sea_orm_active_enums::ReactionType) -> Self {
        match value {
            entity::sea_orm_active_enums::ReactionType::Like => Self::Like,
            entity::sea_orm_active_enums::ReactionType::Dislike => Self::Dislike,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Subject {
    Post(Uuid),
    Comment(i32),
}

impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::Post(id) => write!(f, "post:{}", id),
            Subject::Comment(id) => write!(f, "comment:{}", id),
        }
    }
}

impl FromStr for Subject {
    type Err = SubjectParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, raw_id) = s
            .split_once(':')
            .ok_or(SubjectParseError::InvalidFormat)?;

        match kind {
            "post" => Ok(Self::Post(Uuid::parse_str(raw_id)?)),
            "comment" => Ok(Self::Comment(raw_id.parse()?)),
            _ => Err(SubjectParseError::UnknownKind(kind.to_string())),
        }
    }
}

#[derive(Debug, Error)]
pub enum SubjectParseError {
    #[error("invalid subject format (expected `<kind>:<id>`)")]
    InvalidFormat,

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error(transparent)]
    Int(#[from] ParseIntError),

    #[error("unknown subject kind: {0}")]
    UnknownKind(String),
}

#[derive(Debug, Error)]
pub enum ReactionParseError {
    #[error(transparent)]
    Subject(#[from] SubjectParseError)
}