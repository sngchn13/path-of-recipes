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
}

impl IntoResponse for CommentError {
    fn into_response(self) -> axum::response::Response {
        match self {
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