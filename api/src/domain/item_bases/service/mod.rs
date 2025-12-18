use crate::domain::{item_bases::{ItemBaseServiceError, entity::ItemBase}, item_types::ItemType};

pub mod implement;

#[async_trait::async_trait]
pub trait ItemBaseService: Send + Sync {
    async fn get_item_type_with_bases(&self, item_type_name: &str) -> Result<(ItemType, Vec<ItemBase>), ItemBaseServiceError>;
}
