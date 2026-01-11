use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr)
}

impl IntoResponse for GraphError {
    fn into_response(self) -> axum::response::Response {
        match self {
            err => {
                warn!("{}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            },
        }
    }
}