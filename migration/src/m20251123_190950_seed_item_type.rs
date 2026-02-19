use entity::item_types;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let seed_data = vec![
            "claws", "daggers", "wands", "one_hand_swords", "one_hand_axes", "one_hand_maces", "sceptres", "rune_daggers", "thrusting_one_hand_swords",
            "bows", "staves", "two_hand_swords", "two_hand_axes", "two_hand_maces", "warstaves", "fishing_rods",
            "amulets", "rings", "belts", "trinkets",
            "gloves_str", "gloves_dex", "gloves_int", "gloves_str_dex", "gloves_str_int", "gloves_dex_int",
            "boots_str", "boots_dex", "boots_int", "boots_str_dex", "boots_str_int", "boots_dex_int",
            "body_armours_str", "body_armours_dex", "body_armours_int", "body_armours_str_dex", "body_armours_str_int", "body_armours_dex_int", "body_armours_str_dex_int", 
            "helmets_str", "helmets_dex", "helmets_int", "helmets_str_dex", "helmets_str_int", "helmets_dex_int", 
            "quivers", "shields_str", "shields_dex", "shields_int", "shields_str_dex", "shields_str_int", "shields_dex_int",
            "unset_ring", "bone_ring", "convoking_wand", "bone_spirit_shield", "runic_crown", "runic_sabatons", "runic_gauntlets"
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

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
