use entity::sections;
use sea_orm_migration::{prelude::*};
use sea_orm::{ActiveModelTrait, Set};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();
        
        let section_names = vec![
            "normal", "master", //작업대임. poedb기준으로 표기함
            "shaper", "elder", "crusader", "redeemer", "hunter", "warlord", 
            "delve", "incursion", "veiled",
            "desecrated", "haunted", "infamous", "scourgeup", "scourgedown", "sentinel", "synthesis_corrupted", "warbands",
            "enchant",
            "essence", 
            "bestiary",
            "corrupted", "searing", "eater", "synthesis"
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

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
