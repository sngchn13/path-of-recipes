use std::sync::Arc;

use crate::domain::{item_types::ItemTypeRepository, mod_groups::{ModGroup, repository::ModGroupRepository, service::{ModGroupService, ModGroupServiceError}}};

pub struct ModGroupServiceImpl {
    mod_group_repository: Arc<dyn ModGroupRepository>,
    item_type_repository: Arc<dyn ItemTypeRepository>,
}

impl ModGroupServiceImpl {
    pub fn new(mod_group_repository: Arc<dyn ModGroupRepository>, item_type_repository: Arc<dyn ItemTypeRepository>) -> Self{
        Self { mod_group_repository, item_type_repository }
    }
}

#[async_trait::async_trait]
impl ModGroupService for ModGroupServiceImpl {
    async fn calculate_mod_groups_for_item_type(&self, item_type_name: &str) -> Result<Vec<ModGroup>, ModGroupServiceError> {
        let item_type = self.item_type_repository.find_by_name(item_type_name).await?;
        let item_type = match item_type {
            Some(v) => v,
            None => return Err(ModGroupServiceError::ItemTypeNotFound(item_type_name.to_string())),
        };

        Ok(self.mod_group_repository.find_all_by_item_type(&item_type).await?)
    }
}