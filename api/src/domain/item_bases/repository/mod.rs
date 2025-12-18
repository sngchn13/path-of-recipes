use crate::domain::{item_bases::{ItemBaseRepositoryError, entity::ItemBase}, item_types::ItemType};

pub mod implement;

#[async_trait::async_trait]
pub trait ItemBaseRepository: Send + Sync{
    async fn find_all_by_item_type(&self, item_type: &ItemType) -> Result<Vec<ItemBase>, ItemBaseRepositoryError>;
}
