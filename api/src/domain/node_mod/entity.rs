use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct NodeMod{
    pub node_id: i32,
    pub mod_id: i32,
    pub placement: Placement,
    pub fractured: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Placement {
    Implicit,
    Explicit,
    Enchantment,
    Others,
}

impl Placement {
    pub fn from(value: entity::sea_orm_active_enums::Placement) -> Self {
        match value {
            entity::sea_orm_active_enums::Placement::Implicit => Placement::Implicit,
            entity::sea_orm_active_enums::Placement::Explicit => Placement::Explicit,
            entity::sea_orm_active_enums::Placement::Enchantment => Placement::Enchantment,
            entity::sea_orm_active_enums::Placement::Others => Placement::Others,
        }
    }

    pub fn to(self) -> entity::sea_orm_active_enums::Placement{
        match self {
            Placement::Implicit => entity::sea_orm_active_enums::Placement::Implicit,
            Placement::Explicit => entity::sea_orm_active_enums::Placement::Explicit,
            Placement::Enchantment => entity::sea_orm_active_enums::Placement::Enchantment,
            Placement::Others => entity::sea_orm_active_enums::Placement::Others,
        }
    }
}

impl fmt::Display for Placement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Placement::Implicit => "implicit",
            Placement::Explicit => "explicit",
            Placement::Enchantment => "enchantment",
            Placement::Others => "others",
        };

        write!(f, "{}", s)
    }
}