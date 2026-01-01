use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Node{
    pub id: i32,
    pub context: Option<String>,
    pub rarity: Rarity,
    pub item_level: i32,
    pub note: Option<String>,
    pub position_x: f32,
    pub position_y: f32,
    pub item_base_id: i32,
}

impl Node {
    pub fn new(id: i32, context: Option<String>,rarity: Rarity, item_level: i32, note: Option<String>, position_x: f32, position_y: f32, item_base_id: i32) -> Self{
        Self { id, context, rarity, item_level, note, position_x, position_y, item_base_id }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Normal,
    Magic,
    Rare,
}

impl Rarity {

    pub fn from(value: entity::sea_orm_active_enums::Rarity) -> Self {
        match value {
            entity::sea_orm_active_enums::Rarity::Normal => Rarity::Normal,
            entity::sea_orm_active_enums::Rarity::Magic => Rarity::Magic,
            entity::sea_orm_active_enums::Rarity::Rare => Rarity::Rare,
        }
    }

    pub fn to(self) -> entity::sea_orm_active_enums::Rarity{
        match self {
            Rarity::Normal => entity::sea_orm_active_enums::Rarity::Normal,
            Rarity::Magic => entity::sea_orm_active_enums::Rarity::Magic,
            Rarity::Rare => entity::sea_orm_active_enums::Rarity::Rare,
        }
    }
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rarity::Normal => "normal",
            Rarity::Magic => "magic",
            Rarity::Rare => "rare",
        };

        write!(f, "{}", s)
    }
}