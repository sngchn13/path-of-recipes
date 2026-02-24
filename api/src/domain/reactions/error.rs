use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::{info, warn};
use uuid::Uuid;

use crate::domain::reactions::ReactionParseError;

#[derive(Debug, Error)]
pub enum ReactionError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    
    #[error(transparent)]
    TryFromOrm(#[from] ReactionParseError),

    #[error("Reaction conflicted: anon_id={0}, subject={1}")]
    Conflict(Uuid, String),
}

impl IntoResponse for ReactionError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ReactionError::Conflict(anon_id, subject) => {
                info!("Reaction conflicted: anon_id={}, subject={}", anon_id, subject);
                (StatusCode::CONFLICT, "Reaction already exists.").into_response()
            },
            err => {
                warn!("{}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            }
        }
    }
}