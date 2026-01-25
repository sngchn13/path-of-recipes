use std::{collections::HashSet, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Node{
    pub id: i32,
    pub context: Context,
    pub rarity: Rarity,
    pub item_level: i32,
    pub quality: i32,
    pub flags: Flags,
    pub influences: HashSet<Influence>,
    pub note: Option<String>,
    pub position_x: f32,
    pub position_y: f32,
    pub item_base_id: i32,
}

impl Node {
    pub fn new(id: i32, context: Context,rarity: Rarity, item_level: i32, quality: i32, flags: Flags, influences: HashSet<Influence> ,note: Option<String>, position_x: f32, position_y: f32, item_base_id: i32) -> Self{
        Self { id, context, rarity, item_level, quality, flags, influences, note, position_x, position_y, item_base_id }
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Context{}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Influence {
    Elder, Shaper, Warlord, Crusader, Redeemer, Hunter 
}

impl Influence {

    const ELDER: i32    = 1 << 0;
    const SHAPER: i32   = 1 << 1;
    const WARLORD: i32  = 1 << 2;
    const CRUSADER: i32 = 1 << 3;
    const REDEEMER: i32 = 1 << 4;
    const HUNTER: i32   = 1 << 5;

    fn convert(&self) -> i32 {
        match self {
            Influence::Elder => Self::ELDER,
            Influence::Shaper => Self::SHAPER,
            Influence::Warlord => Self::WARLORD,
            Influence::Crusader => Self::CRUSADER,
            Influence::Redeemer => Self::REDEEMER,
            Influence::Hunter => Self::HUNTER,
        }
    }

    pub fn from_bits(bits: i32) -> HashSet<Self> {
        if bits < 0 {
            return HashSet::new();
        }

        let mut res = HashSet::new();

        if bits & Self::ELDER != 0 {
            res.insert(Self::Elder);
        }
        if bits & Self::SHAPER != 0 {
            res.insert(Self::Shaper);
        }
        if bits & Self::WARLORD != 0 {
            res.insert(Self::Warlord);
        }
        if bits & Self::CRUSADER != 0 {
            res.insert(Self::Crusader);
        }
        if bits & Self::REDEEMER != 0 {
            res.insert(Self::Redeemer);
        }
        if bits & Self::HUNTER != 0 {
            res.insert(Self::Hunter);
        }

        res
    }

    pub fn to_bits(influences: &HashSet<Influence>) -> i32 {
        let mut res = 0;
        for influence in influences {
            res = res | influence.convert();
        }

        res
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Flags {
    pub corrupted: bool,
    pub mirrored: bool,
    pub split: bool,
}

impl Flags {
    pub fn new(corrupted: bool, mirrored: bool, split: bool) -> Self {
        Self { corrupted, mirrored, split }
    }

    pub fn from_bits(bits: i32) -> Self {
        Self { 
            corrupted: bits & 1 != 0, 
            mirrored: bits & 2 != 0, 
            split: bits & 4 != 0, 
        }
    }

    pub fn to_bits(&self) -> i32 {
        (self.corrupted as i32) << 1
        | (self.mirrored as i32) << 2
        | (self.split as i32) << 3
    }
}

