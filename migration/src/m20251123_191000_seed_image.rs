use std::{env, path::PathBuf};

use entity::images;
use sea_orm_migration::{prelude::*, sea_orm::{ActiveValue::Set, EntityTrait}};
use serde::Deserialize;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();
        let mut models = Vec::new();

        let resources_directory = env::var("RESOURCES_DIRECTORY").expect("RESOURCES_DIRECTORY is not set in .env file");
        let image_path = PathBuf::from(resources_directory).join("csv").join("image").join("images.csv");
        let mut reader = csv::Reader::from_path(image_path).expect("images.csv doesn't exist");

        for result in reader.deserialize() {
            if let Ok(image_record) = result {
                let image_record:ImageRecord = image_record;
                let id = Uuid::new_v4();

                let model = images::ActiveModel {
                    id: Set(id),
                    relative_path: Set(image_record.relative_path),
                    name: Set(image_record.name),
                    width: Set(image_record.width),
                    height: Set(image_record.height),
                    size: Set(image_record.size),
                    extension: Set(image_record.extension),
                    desc: Set(image_record.desc),
                };

                models.push(model);
            }
        }

        images::Entity::insert_many(models).exec(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();
        
        images::Entity::delete_many()
            .exec(db)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct ImageRecord {
    name: String,
    relative_path: String,
    width: i32,
    height: i32,
    size: i32,
    extension: String,
    desc: String,
}