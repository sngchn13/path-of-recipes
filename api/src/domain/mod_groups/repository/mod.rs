use crate::domain::mod_groups::{ModGroup, error::ModGroupRepositoryError};

pub mod implement;
pub use implement::ModGroupRepositoryImpl;

#[async_trait::async_trait]
pub trait ModGroupRepository: Send + Sync{
    async fn find_all_by_item_type(&self, item_type_name: &str) -> Result<Vec<ModGroup>, ModGroupRepositoryError>;
}
