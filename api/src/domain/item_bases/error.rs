use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::domain::item_types::error::ItemTypeRepositoryError;


#[derive(Debug, Error)]
pub enum ItemBaseHandlerError {
    #[error(transparent)]
    Db {
        source: ItemBaseServiceError,
    },

    #[error(transparent)]
    ItemTypeRepositoryErr {
        source: ItemBaseServiceError,
    },

    #[error(transparent)]
    ItemTypeNotFound{
        source: ItemBaseServiceError,
    },

    #[error("missing required parameter: {0}")]
    MissingRequiredParameter(String)    
}

impl IntoResponse for ItemBaseHandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ItemBaseHandlerError::Db { ..} => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response(),
            ItemBaseHandlerError::ItemTypeRepositoryErr { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response(),
            ItemBaseHandlerError::MissingRequiredParameter(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            ItemBaseHandlerError::ItemTypeNotFound { .. } => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
        }
    }
}

impl From<ItemBaseServiceError> for ItemBaseHandlerError {
    fn from(err: ItemBaseServiceError) -> Self {
        match err {
            ItemBaseServiceError::Db { .. } => ItemBaseHandlerError::Db { source: err },
            ItemBaseServiceError::ItemTypeRepositoryErr { .. } => ItemBaseHandlerError::ItemTypeRepositoryErr { source: err },
            ItemBaseServiceError::ItemTypeNotFound(_) => ItemBaseHandlerError::ItemTypeNotFound { source: err },
        }
    }
}

#[derive(Debug, Error)]
pub enum ItemBaseServiceError {
    #[error(transparent)]
    Db {
        source: ItemBaseRepositoryError,
    },

    #[error(transparent)]
    ItemTypeRepositoryErr(#[from] ItemTypeRepositoryError),

    #[error("{0} doesn't exist")]
    ItemTypeNotFound(String)
}

impl From<ItemBaseRepositoryError> for ItemBaseServiceError {
    fn from(err: ItemBaseRepositoryError) -> Self {
        match err {
            ItemBaseRepositoryError::Db(_) => ItemBaseServiceError::Db { source: err },
        }
    }
}

#[derive(Debug, Error)]
pub enum ItemBaseRepositoryError {
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),

}