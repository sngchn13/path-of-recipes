use entity::images;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::{domain::images::{Image, ImageRepositoryError, repository::ImageRepository}, shared::to_image};



pub struct ImageRepositoryImpl {
    database_connection: DatabaseConnection,
}

impl ImageRepositoryImpl {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self { database_connection }
    }
}

#[async_trait::async_trait]
impl ImageRepository for ImageRepositoryImpl {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Image>, ImageRepositoryError>{
        let image_model = images::Entity::find_by_id(*id).one(&self.database_connection).await?;
        let optional_image = match image_model {
            Some(im) => Some(to_image(im)),
            None => None,
        };

        Ok(optional_image)
    }
}