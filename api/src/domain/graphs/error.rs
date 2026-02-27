use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::warn;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error("Delete graph failed: expected 1 affected row, got {0} (post_id={1})")]
    UnexpectedAffectedRows(u64, Uuid)
}

impl IntoResponse for GraphError {
    fn into_response(self) -> axum::response::Response {
        match self {
            GraphError::UnexpectedAffectedRows(affected_rows, post_id) => {
                warn!("Delete graph failed: expected 1 affected row, got {} (post_id={})", affected_rows, post_id);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete graph.").into_response()
            },
            err => {
                warn!("{}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            },
        }
    }
}