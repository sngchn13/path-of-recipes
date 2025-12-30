use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeRepositoryError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error("Item base not found for node_id ({0})")]
    ItemBaseNotFound(String),

    #[error("Item type not found for item_base_id ({0})")]
    ItemTypeNotFound(String),
}