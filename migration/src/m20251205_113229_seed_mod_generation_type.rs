use std::{env, path::PathBuf};

use entity::{mod_generation_types};
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
        let mod_generation_type_path = PathBuf::from(resources_directory).join("csv").join("mod_generation_type").join("mod_generation_types.csv");
        let mut reader = csv::Reader::from_path(mod_generation_type_path).expect("mod_generation_types.csv doesn't exist");

        for result in reader.deserialize() {
            if let Ok(mod_generation_type_record) = result {
                let mod_generation_type_record: ModGenerationTypeRecord = mod_generation_type_record;
        
                let model = mod_generation_types::ActiveModel{
                    id: Set(mod_generation_type_record.id),
                    label: Set(mod_generation_type_record.label),
                };

                models.push(model);
            }
        }

        mod_generation_types::Entity::insert_many(models)
            .exec(db)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();
        mod_generation_types::Entity::delete_many()
            .exec(db)
            .await?;        

        Ok(())
    }
}


#[derive(Debug, Deserialize)]
struct ModGenerationTypeRecord {
    id: i32,
    label: Option<String>,
}
