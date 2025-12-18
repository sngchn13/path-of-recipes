use crate::domain::item_types::{ItemType, error::ItemTypeRepositoryError};

pub mod implement;

#[async_trait::async_trait]
pub trait ItemTypeRepository: Send + Sync{
    async fn find_by_name(&self, name: &str) -> Result<Option<ItemType>, ItemTypeRepositoryError>;
}