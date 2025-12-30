use std::collections::HashSet;

use time::OffsetDateTime;
use crate::domain::{item_types::ItemType, mod_groups::{Family, ModGenerationType, Section, Tag}, node_mod::Placement};


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnplacedModAggregate{
    pub id: i32,
    pub name_en: String,
    pub name_kor: String,
    pub display_en: String,
    pub display_kor: String,
    pub req_item_level: i32,
    pub weight: Option<i16>,
    pub tier: Option<i16>,
    pub deprecated_version: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
    pub mod_group_id: i32,
    pub mod_group_value: String,
    pub mod_generation_type: ModGenerationType,
    pub item_type: ItemType,
    pub tags: HashSet<Tag>,
    pub families: HashSet<Family>,
    pub section: Section,
}

impl UnplacedModAggregate {
    pub fn place_mode(self, placement: Placement) -> ModAggregate {
        ModAggregate{
            id: self.id,
            name_en: self.name_en,
            name_kor: self.name_kor,
            display_en: self.display_en,
            display_kor: self.display_kor,
            req_item_level: self.req_item_level,
            weight: self.weight,
            tier: self.tier,
            deprecated_version: self.deprecated_version,
            created_at: self.created_at,
            updated_at: self.updated_at,
            mod_group_id: self.mod_group_id,
            mod_group_value: self.mod_group_value,
            mod_generation_type: self.mod_generation_type,
            item_type: self.item_type,
            tags: self.tags,
            families: self.families,
            section: self.section,
            placement,
        } 
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModAggregate{
    pub id: i32,
    pub name_en: String,
    pub name_kor: String,
    pub display_en: String,
    pub display_kor: String,
    pub req_item_level: i32,
    pub weight: Option<i16>,
    pub tier: Option<i16>,
    pub deprecated_version: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
    pub mod_group_id: i32,
    pub mod_group_value: String,
    pub mod_generation_type: ModGenerationType,
    pub item_type: ItemType,
    pub tags: HashSet<Tag>,
    pub families: HashSet<Family>,
    pub section: Section,
    pub placement: Placement,
}