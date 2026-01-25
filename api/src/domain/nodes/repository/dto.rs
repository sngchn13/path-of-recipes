use std::{collections::HashSet, hash::Hash};

use crate::domain::{item_bases::ItemBase, item_types::ItemType, mods::ModAggregate, nodes::{Context, Flags, Influence, Node, Rarity}};

#[derive(Debug)]
pub struct RawNodeAggregate {
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
    pub item_base_name: String,
    pub item_base_display_en: Option<String>,
    pub item_base_display_kor: Option<String>,
    pub item_type_id: i32,
    pub item_type_name: String,
}

impl RawNodeAggregate {
    pub fn assembly(node: Node, item_base: ItemBase, item_type: ItemType) -> Self {
        Self { 
            id: node.id, 
            context: node.context, 
            rarity: node.rarity, 
            item_level: node.item_level,
            quality: node.quality,
            flags: node.flags,
            influences: node.influences, 
            note: node.note, 
            position_x: node.position_x, 
            position_y: node.position_y, 
            item_base_id: item_base.id, 
            item_base_name: item_base.name, 
            item_base_display_en: item_base.display_en, 
            item_base_display_kor: item_base.display_kor, 
            item_type_id: item_type.id, 
            item_type_name: item_type.name,
        }
    }

    pub fn add_mods(self, mods: Vec<ModAggregate>)-> NodeAggregate{
        NodeAggregate {
            id: self.id,
            context: self.context,
            rarity: self.rarity,
            item_level: self.item_level,
            quality: self.quality,
            flags: self.flags,
            influences: self.influences,
            note: self.note,
            position_x: self.position_x,
            position_y: self.position_y,
            item_base_id: self.item_base_id,
            item_base_name: self.item_base_name,
            item_base_display_en: self.item_base_display_en,
            item_base_display_kor: self.item_base_display_kor,
            item_type_id: self.item_type_id,
            item_type_name: self.item_type_name,
            mods: mods,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeAggregate {
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
    pub item_base_name: String,
    pub item_base_display_en: Option<String>,
    pub item_base_display_kor: Option<String>,
    pub item_type_id: i32,
    pub item_type_name: String,
    pub mods: Vec<ModAggregate>
}

impl Eq for NodeAggregate { }
impl PartialEq for NodeAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Hash for NodeAggregate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}