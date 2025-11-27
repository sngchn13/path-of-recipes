use entity::tags;
use sea_orm_migration::{prelude::*, sea_orm::EntityTrait};
use sea_orm::{ActiveModelTrait, Set};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        let tag_names = vec![
            "physical", "lightning", "cold", "fire", "chaos", "elemental",
            "life", "mana", "armour", "evasion", "energy_shield",
            "attack", "caster", 
            "minion", "aura", "ailment", "curse",
            "defences", "damage", 
            "speed", "critical", "attribute", "resistance", "gem", "drop",
        ];

        for tag_name in tag_names {
          let model = tags::ActiveModel{
            name: Set(tag_name.to_string()),
            ..Default::default()
          };

          model.insert(db).await?;
        }

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
