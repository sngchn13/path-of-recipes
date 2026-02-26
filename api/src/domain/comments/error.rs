use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum CommentError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error(transparent)]
    BcryptError(#[from] BcryptError),

    #[error("invalid password format")]
    InvalidPassword,

    #[error("parent comment({0}) not found")]
    ParentCommentNotFound(i32),

    #[error("comment({0}) not found")]
    CommentNotFound(i32),
}

impl IntoResponse for CommentError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CommentError::InvalidPassword => {
                (StatusCode::BAD_REQUEST, "invalid credentials").into_response()
            },
            CommentError::CommentNotFound(comment_id) => {
                let msg = format!("comment not found: comment_id = {}", comment_id);
                info!("{}", &msg);
                (StatusCode::NOT_FOUND, msg).into_response()
            },
            CommentError::ParentCommentNotFound(comment_id) => {
                let msg = format!("parent comment not found: comment_id = {}", comment_id);
                info!("{}", &msg);
                (StatusCode::BAD_REQUEST, msg).into_response()
            },
            err => {
                warn!("{}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            }
        }
    }
}