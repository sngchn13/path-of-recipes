use std::{env, path::PathBuf};

use entity::tags;
use sea_orm_migration::{prelude::*, sea_orm::{EntityTrait}};
use sea_orm::{Set};
use ::serde::Deserialize;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();
        let mut models = Vec::new();

        let resources_directory = env::var("RESOURCES_DIRECTORY").expect("RESOURCES_DIRECTORY is not set in .env file");
        let tag_path = PathBuf::from(resources_directory).join("csv").join("tag").join("tags.csv");
        let mut reader = csv::Reader::from_path(tag_path).expect("tags.csv doesn't exist");

        for result in reader.deserialize() {
            if let Ok(tag_record) = result {
                let tag_record: TagRecord = tag_record;

                let model = tags::ActiveModel {
                    name: Set(tag_record.name),
                    ..Default::default()
                };

                models.push(model);
            }
        }

        tags::Entity::insert_many(models).exec(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        tags::Entity::delete_many()
            .exec(db)
            .await?;

        Ok(())
    }
}


#[derive(Debug, Deserialize)]
struct TagRecord {
    name: String
}