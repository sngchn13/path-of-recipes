use entity::images;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::domain::images::{Image, ImageRepositoryError, repository::{ImageRepository, implement::orm_to_domain::to_image}};



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

pub mod orm_to_domain {
    use entity::{ images::Model as ImageOrm };
    use crate::domain::images::Image;

    pub fn to_image(orm: ImageOrm) -> Image{
        Image { id: orm.id, relative_path: orm.relative_path, name: orm.name, width: orm.width, height: orm.height, size: orm.size, extension: orm.extension, desc: orm.desc }
    }
}