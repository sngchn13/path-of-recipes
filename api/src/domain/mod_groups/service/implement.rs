use std::sync::Arc;

use crate::domain::mod_groups::{ModGroup, repository::ModGroupRepository, service::{ModGroupService, ModGroupServiceError}};

pub struct ModGroupServiceImpl {
    mod_group_repository: Arc<dyn ModGroupRepository>
}

impl ModGroupServiceImpl {
    pub fn new(mod_group_repository: Arc<dyn ModGroupRepository>) -> Self{
        Self { mod_group_repository }
    }
}

#[async_trait::async_trait]
impl ModGroupService for ModGroupServiceImpl {
    async fn calculate_mod_groups_for_item_type(&self, item_type_name: &str) -> Result<Vec<ModGroup>, ModGroupServiceError> {
        Ok(self.mod_group_repository.find_all_by_item_type(item_type_name).await?)
    }
}