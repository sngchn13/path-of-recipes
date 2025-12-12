use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::domain::{mod_groups::ModGroup, mods::Mod};

#[derive(Deserialize, Serialize)]
pub struct ModGroupResponse{
    pub id: i32,
    pub value: String,
    pub mod_generation_type_id: i32,
    pub item_type: String,
    pub tags: HashSet<String>,
    pub families: HashSet<String>,
    pub section: String,
    pub mods: Vec<Mod>,
}

impl From<ModGroup> for ModGroupResponse {
    fn from(mg: ModGroup) -> Self {
        let tags: HashSet<String> = mg.tags.into_iter().map(|t| t.name).collect();
        let families: HashSet<String> = mg.families.into_iter().map(|f| f.name).collect();

        ModGroupResponse {
            id: mg.id,
            value: mg.value,
            mod_generation_type_id: mg.mod_generation_type.id,
            item_type: mg.item_type.name,
            tags,
            families,
            section: mg.section.name,
            mods: mg.mods,
        }
    }
}