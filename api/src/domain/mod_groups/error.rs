use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ModGroupHandlerError {
    #[error(transparent)]
    Db {
        source: ModGroupServiceError,
    },

    #[error(transparent)]
    ItemTypeNotFound {
        source: ModGroupServiceError,
    },

    #[error(transparent)]
    AssembleError {
        source: ModGroupServiceError,
    },

    #[error("missing required parameter: {0}")]
    MissingRequiredParameter(String)    
}

impl IntoResponse for ModGroupHandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ModGroupHandlerError::Db { ..} => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response(),
            ModGroupHandlerError::ItemTypeNotFound { .. } => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            ModGroupHandlerError::AssembleError { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response(),
            ModGroupHandlerError::MissingRequiredParameter(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
        }
    }
}

impl From<ModGroupServiceError> for ModGroupHandlerError {
    fn from(err: ModGroupServiceError) -> Self {
        match err {
            ModGroupServiceError::Db{source: _} => ModGroupHandlerError::Db { source: err },
            ModGroupServiceError::ItemTypeNotFound{source: _} => ModGroupHandlerError::ItemTypeNotFound { source: err },
            ModGroupServiceError::AssembleError{source: _} => ModGroupHandlerError::AssembleError { source: err },
        }
    }
}

#[derive(Debug, Error)]
pub enum ModGroupServiceError {
    #[error(transparent)]
    Db {
        source: ModGroupRepositoryError,
    },

    #[error(transparent)]
    ItemTypeNotFound {
        source: ModGroupRepositoryError,
    },

    #[error(transparent)]
    AssembleError {
        source: ModGroupRepositoryError,
    },
}

impl From<ModGroupRepositoryError> for ModGroupServiceError {
    fn from(err: ModGroupRepositoryError) -> Self {
        match err {
            ModGroupRepositoryError::Db(_) => ModGroupServiceError::Db { source: err },
            ModGroupRepositoryError::ItemTypeNotFound(_) => ModGroupServiceError::ItemTypeNotFound { source: err },
            ModGroupRepositoryError::AssembleError(_) => ModGroupServiceError::AssembleError { source: err },
        }
    }
}

#[derive(Debug, Error)]
pub enum ModGroupRepositoryError {
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),

    #[error("not found item type: {0}")]
    ItemTypeNotFound(String),

    #[error("assemble fail: {0}")]
    AssembleError(String),
}