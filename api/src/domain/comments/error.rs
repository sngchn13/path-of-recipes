use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum CommentError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error(transparent)]
    InvalidAnonId(#[from] uuid::Error),

    #[error(transparent)]
    BcryptError(#[from] BcryptError),

    #[error("parent comment({0}) not found")]
    ParentCommentNotFound(String),
}

impl IntoResponse for CommentError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CommentError::ParentCommentNotFound(comment_id) => {
                let msg = format!("parent comment not found: comment_id = {}", comment_id);
                info!("{}", &msg);
                (StatusCode::BAD_REQUEST, msg).into_response()
            },
            CommentError::InvalidAnonId(err) => {
                info!("{}", err);
                (StatusCode::BAD_REQUEST, "invalid anon_id").into_response()
            },
            err => {
                warn!("{}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            }
        }
    }
}