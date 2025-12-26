use thiserror::Error;

#[derive(Debug, Error)]
pub enum  EdgeRepositoryError{
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error("cause name({0}) not found")]
    CauseNameNotFound(String)
}