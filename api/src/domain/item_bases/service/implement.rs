use std::sync::Arc;

use crate::domain::{item_bases::{ItemBaseRepository, ItemBaseServiceError, entity::ItemBase, service::ItemBaseService}, item_types::{ItemType, ItemTypeRepository}};


pub struct ItemBaseServiceImpl {
    item_base_repository: Arc<dyn ItemBaseRepository>,
    item_type_repository: Arc<dyn ItemTypeRepository>,
}

impl ItemBaseServiceImpl {
    pub fn new(item_base_repository: Arc<dyn ItemBaseRepository>, item_type_repository: Arc<dyn ItemTypeRepository>) -> Self{
        Self { item_base_repository, item_type_repository }
    }
}

#[async_trait::async_trait]
impl ItemBaseService for ItemBaseServiceImpl {
    async fn get_item_type_with_bases(&self, item_type_name: &str) -> Result<(ItemType, Vec<ItemBase>), ItemBaseServiceError>
    {
        let item_type = self.item_type_repository.find_by_name(&item_type_name).await?;
        let item_type = match item_type {
            Some(v) => v,
            None => return Err(ItemBaseServiceError::ItemTypeNotFound(item_type_name.to_string())),
        };
        let item_bases = self.item_base_repository.find_all_by_item_type(&item_type).await?;
        
        Ok((item_type, item_bases))
    }
}