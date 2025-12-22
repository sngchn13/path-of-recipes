use uuid::Uuid;

use crate::domain::images::{Image, ImageRepositoryError};

pub mod implement;

pub use implement::ImageRepositoryImpl;

#[async_trait::async_trait]
pub trait ImageRepository: Send + Sync{
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Image>, ImageRepositoryError>;
}