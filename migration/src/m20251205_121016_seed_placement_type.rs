use entity::placement_types;
use sea_orm_migration::{prelude::*, sea_orm::{ActiveValue::Set, EntityTrait}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();
        let placement_types = vec!["IMPLICIT", "EXPLICIT", "ENCHANTMENT"];
        let mut models = Vec::new();

        for placement_type in placement_types {
            let model = placement_types::ActiveModel{
                id: Set(placement_type.to_string()),
            };

            models.push(model);
        }

        placement_types::Entity::insert_many(models).exec(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();
        placement_types::Entity::delete_many().exec(db).await?;

        Ok(())
    }
}