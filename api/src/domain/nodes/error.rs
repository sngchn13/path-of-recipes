use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum NodeError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error("Item base not found for node_id ({0})")]
    ItemBaseNotFound(String),

    #[error("Item type not found for item_base_id ({0})")]
    ItemTypeNotFound(String),
}

impl IntoResponse for NodeError {
    fn into_response(self) -> axum::response::Response {
        match self {
            err => {
                warn!("{}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            },
        }
    }
}