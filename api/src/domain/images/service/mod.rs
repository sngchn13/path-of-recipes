use crate::domain::images::{Image, ImageServiceError};

pub mod implement;

pub use implement::ImageServiceImpl;

#[async_trait::async_trait]
pub trait ImageService: Send + Sync {
    async fn get_image_by_id(&self, id: &str) -> Result<Image, ImageServiceError>;
}
