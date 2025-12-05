use std::{env, path::PathBuf};

use entity::families;
use sea_orm_migration::{prelude::*, sea_orm::{ActiveValue::Set, EntityTrait}};
use serde::Deserialize;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();
        let mut models = Vec::new();

        let resources_directory = env::var("RESOURCES_DIRECTORY").expect("RESOURCES_DIRECTORY is not set in .env file");
        let family_path = PathBuf::from(resources_directory).join("csv").join("family").join("families.csv");
        let mut reader = csv::Reader::from_path(family_path).expect("families.csv doesn't exist");

        for result in reader.deserialize() {
            if let Ok(family_record) = result {
                let family_record: FamilyRecord = family_record;

                let model = families::ActiveModel{
                    name: Set(family_record.name),
                    ..Default::default()
                };

                models.push(model);
            }
        }

        families::Entity::insert_many(models).exec(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        families::Entity::delete_many()
            .exec(db)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct FamilyRecord {
    name: String
}
