use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeModRepositoryError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr)
}