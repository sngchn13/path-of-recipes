use entity::sections;
use sea_orm_migration::{prelude::*, sea_orm::EntityTrait};
use sea_orm::{ActiveModelTrait, Set};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();
        
        let section_names = vec![
            "normal", 
            "the_shaper", "the_elder", "the_crusader", "the_redeemer", "the_hunter", "the_warlord", 
            "delve", "incursion", "veiled", 
            "crafting_bench", 
            "essence", 
            "bestiary",
            "corrupted", "the_searing_exarch", "the_eater_of_worlds", "synthesis"
        ];

        for section_name in section_names {
            let  model = sections::ActiveModel{
                name: Set(section_name.to_string()),
                ..Default::default()
            };
            
            model.insert(db).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        sections::Entity::delete_many()
            .exec(db)
            .await?;

        Ok(())
    }
}
