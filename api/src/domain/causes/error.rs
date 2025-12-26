use thiserror::Error;

#[derive(Debug, Error)]
pub enum CauseRepositoryError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr)  
}