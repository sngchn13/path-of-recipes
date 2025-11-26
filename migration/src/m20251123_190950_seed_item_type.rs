use entity::item_types;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let seed_data = vec![
            "claws", "daggers", "wands", "convoking_wands","one_hand_swords", "one_hand_axes", "one_hand_maces", "sceptres", "rune_daggers", "thrusting_one_hand_swords",
            "bows", "staves", "two_hand_swords", "two_hand_axes", "two_hand_maces", "warstaves", "fishing_rods",
            "amulets", "rings", "bone_rings", "belts",
            "gloves(str)", "gloves(dex)", "gloves(int)", "gloves(str_dex)", "gloves(str_int)", "gloves(dex_int)",
            "boots(str)", "boots(dex)", "boots(int)", "boots(str_dex)", "boots(str_int)", "boots(dex_int)",
            "body_armours(str)", "body_armours(dex)", "body_armours(int)", "body_armours(str_dex)", "body_armours(str_int)", "body_armours(dex_int)", "body_armours(str_dex_int)", 
            "helmets(str)", "helmets(dex)", "helmets(int)", "helmets(str_dex)", "helmets(str_int)", "helmets(dex_int)", 
            "quivers", "shields(str)", "shields(dex)", "shields(int)", "shields(str_dex)", "shields(str_int)", "shields(dex_int)",
        ];

        for name in seed_data {
            let model = item_types::ActiveModel{
                name: Set(name.to_string()),
                ..Default::default()
            };

            model.insert(db).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        item_types::Entity::delete_many()
            .exec(db)
            .await?;

        Ok(())
    }
}
