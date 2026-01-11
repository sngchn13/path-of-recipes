use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum ModError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error("Failed to find a mod_group for mod id '{0}'.")]
    ModGroupNotFound(String),

    #[error("Failed to find a mod_generation_type for mod_group_id '{0}'.")]
    ModGenerationTypeNotFound(String),

    #[error("Failed to find a section for mod_group_id '{0}'.")]
    SectionNotFound(String),

    #[error("Failed to find a section for mod_group_id '{0}'.")]
    ItemTypeNotFound(String),
}

impl IntoResponse for ModError {
    fn into_response(self) -> axum::response::Response {
        match self {
            err => {
                warn!("{}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            },
        }
    }
}