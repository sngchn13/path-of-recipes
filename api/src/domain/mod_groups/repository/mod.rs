use crate::domain::{item_types::ItemType, mod_groups::{ModGroup, error::ModGroupRepositoryError}};

pub mod implement;
pub use implement::ModGroupRepositoryImpl;

#[async_trait::async_trait]
pub trait ModGroupRepository: Send + Sync{
    async fn find_all_by_item_type(&self, item_type: &ItemType) -> Result<Vec<ModGroup>, ModGroupRepositoryError>;
}
