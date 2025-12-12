pub mod implement;

pub use implement::ModGroupServiceImpl;

use crate::domain::mod_groups::{ModGroup, error::ModGroupServiceError};

#[async_trait::async_trait]
pub trait ModGroupService: Send + Sync {
    async fn calculate_mod_groups_for_item_type(&self, item_type_name: &str) -> Result<Vec<ModGroup>, ModGroupServiceError>;
}
