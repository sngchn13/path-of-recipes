use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphRepositoryError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr)
}