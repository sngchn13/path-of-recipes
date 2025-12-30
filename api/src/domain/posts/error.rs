use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::domain::{causes::CauseRepositoryError, edges::EdgeRepositoryError, graphs::GraphRepositoryError, mods::ModRepositoryError, node_mod::NodeModRepositoryError, nodes::NodeRepositoryError};


#[derive(Debug, Error)]
pub enum PostHandlerError {
    #[error(transparent)]
    BadRequestErr(#[from] PostServiceError)
}

impl IntoResponse for PostHandlerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
    }
}

#[derive(Debug, Error)]
pub enum PostServiceError {
    #[error(transparent)]
    TransactionMakeFail(#[from] sea_orm::DbErr),

    #[error(transparent)]
    PostRepositoryErr(#[from] PostRepositoryError),

    #[error(transparent)]
    GraphRepositoryErr(#[from] GraphRepositoryError),

    #[error(transparent)]
    NodeRepositoryErr(#[from] NodeRepositoryError),

    #[error(transparent)]
    EdgeRepositoryErr(#[from] EdgeRepositoryError),

    #[error(transparent)]
    CauseRepositoryErr(#[from] CauseRepositoryError),
    
    #[error(transparent)]
    NodeModRepositoryErr(#[from] NodeModRepositoryError),

    #[error(transparent)]
    ModRepositoryErr(#[from] ModRepositoryError),

    #[error("post_id({0}) not found")]
    NotFoundPost(String),

    #[error("graph not found with post_id({0})")]
    NotFoundGraph(String),

    #[error("{0}")]
    AssembleErr(String)
}

#[derive(Debug, Error)]
pub enum PostRepositoryError {
    #[error(transparent)]
    CryptError(#[from] bcrypt::BcryptError),

    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
}