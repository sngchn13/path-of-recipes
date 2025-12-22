use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ImageHandlerError {
    #[error(transparent)]
    Db {
        source: ImageServiceError,
    },

    #[error(transparent)]
    ImageNotFound{
        source: ImageServiceError,
    },

    #[error(transparent)]
    InvalidUuid{
        source: ImageServiceError,
    },

    #[error("tower error")]
    TowerError,
}

impl IntoResponse for ImageHandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ImageHandlerError::Db { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response(),
            ImageHandlerError::ImageNotFound { .. } => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            ImageHandlerError::InvalidUuid { .. } => (StatusCode::BAD_REQUEST, "invalid uuid").into_response(),
            ImageHandlerError::TowerError => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response(),
        }
    }
}

impl From<ImageServiceError> for ImageHandlerError {
    fn from(err: ImageServiceError) -> Self {
        match err {
            ImageServiceError::Db { .. } => ImageHandlerError::Db { source: err },
            ImageServiceError::ImageNotFound(_) => ImageHandlerError::ImageNotFound { source: err },
            ImageServiceError::InvalidUuid(_) => ImageHandlerError::InvalidUuid { source: err },
        }
    }
}

#[derive(Debug, Error)]
pub enum ImageServiceError {
    #[error(transparent)]
    Db {
        source: ImageRepositoryError,
    },

    #[error("{0} doesn't exist")]
    ImageNotFound(String),

    #[error("invalid uuid: {0}")]
    InvalidUuid(String)
}

impl From<ImageRepositoryError> for ImageServiceError {
    fn from(err: ImageRepositoryError) -> Self {
        match err {
            ImageRepositoryError::Db(_) => ImageServiceError::Db { source: err }
        }
    }
}

#[derive(Debug, Error)]
pub enum ImageRepositoryError {
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
}