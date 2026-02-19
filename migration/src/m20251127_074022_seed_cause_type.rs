use entity::cause_types;
use sea_orm_migration::{prelude::*};
use sea_orm::{ActiveModelTrait, Set};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        let cause_type_names = vec![
            "orb", "essence", "fossil", "crafting_bench", "horticrafting"
        ];

        for cause_type_name in cause_type_names {
            let model = cause_types::ActiveModel{
                name: Set(cause_type_name.to_string()),
                ..Default::default()
            };

            model.insert(db).await?;
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

