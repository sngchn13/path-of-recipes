use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeRepositoryError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr)
}