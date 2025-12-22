use std::sync::Arc;

use uuid::Uuid;

use crate::domain::images::{Image, ImageRepository, ImageServiceError, service::ImageService};

pub struct ImageServiceImpl {
    image_reposiroy: Arc<dyn ImageRepository>,
}

impl ImageServiceImpl {
    pub fn new(image_reposiroy: Arc<dyn ImageRepository>) -> Self{
        Self { image_reposiroy }
    }
}

#[async_trait::async_trait]
impl ImageService for ImageServiceImpl {
    async fn get_image_by_id(&self, id: &str) -> Result<Image, ImageServiceError>{
        let uuid = Uuid::parse_str(id).map_err(|_| ImageServiceError::InvalidUuid(id.to_string()))?;
        let image = self.image_reposiroy.find_by_id(&uuid).await?;

        match image {
            Some(v) =>Ok(v),
            None => Err(ImageServiceError::ImageNotFound(uuid.to_string())),
        }
    }
}