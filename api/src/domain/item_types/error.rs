use thiserror::Error;

#[derive(Debug, Error)]
pub enum ItemTypeRepositoryError {
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
}