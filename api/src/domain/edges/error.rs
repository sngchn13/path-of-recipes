use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum  EdgeError{
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error("cause name({0}) not found")]
    CauseNameNotFound(String)
}

impl IntoResponse for EdgeError {
    fn into_response(self) -> axum::response::Response {
        match self {
            EdgeError::CauseNameNotFound(cause_name) => {
                let msg = format!("cause name not found: {}", cause_name);
                (StatusCode::BAD_REQUEST, msg).into_response()
            },
            err => {
                warn!("{}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            },
        }
    }
}