use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::{info, warn};

use crate::domain::{causes::CauseError, edges::EdgeError, graphs::GraphError, mods::ModError, node_mod::NodeModError, nodes::NodeError};


#[derive(Debug, Error)]
pub enum PostError {

    #[error(transparent)]
    CryptError(#[from] bcrypt::BcryptError),

    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error(transparent)]
    GraphErr(#[from] GraphError),

    #[error(transparent)]
    NodeErr(#[from] NodeError),

    #[error(transparent)]
    EdgeErr(#[from] EdgeError),

    #[error(transparent)]
    CauseErr(#[from] CauseError),
    
    #[error(transparent)]
    NodeModErr(#[from] NodeModError),

    #[error(transparent)]
    ModErr(#[from] ModError),

    #[error("post_id({0}) not found")]
    NotFoundPost(String),

    #[error("graph is missing for post_id({0})")]
    GraphMissingForPost(String),

    #[error("{0}")]
    AssembleErr(String),

    #[error("invalid password")]
    InvalidPasswordErr,
}

impl IntoResponse for PostError {
    fn into_response(self) -> axum::response::Response {
        match self {
            PostError::NotFoundPost(post_id) => {
                let msg = format!("post not found: post_id={}", post_id);
                info!("{}", &msg);
                (StatusCode::BAD_REQUEST, msg).into_response()
            },
            PostError::InvalidPasswordErr => {
                (StatusCode::UNAUTHORIZED).into_response()
            },
            PostError::EdgeErr(err) => err.into_response(),
            PostError::GraphErr(err) => err.into_response(),
            PostError::NodeErr(err) => err.into_response(),
            PostError::CauseErr(err) => err.into_response(),
            PostError::NodeModErr(err) => err.into_response(),
            PostError::ModErr(err) => err.into_response(),
            err => {
                warn!("{}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            },
            }
    }
}
