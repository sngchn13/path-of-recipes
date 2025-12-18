use serde::{Deserialize, Serialize};

use crate::domain::{item_bases::entity::ItemBase, item_types::ItemType};


#[derive(Deserialize, Serialize)]
pub struct ItemBaseResponse {
    pub id: i32,
    pub name: String,
    pub display_en: Option<String>,
    pub display_kor: Option<String>,
    pub item_type_name: String,
}

pub fn item_base_to_item_base_response(item_base: ItemBase, item_type: &ItemType) -> ItemBaseResponse {
    ItemBaseResponse { id: item_base.id, name: item_base.name, display_en: item_base.display_en, display_kor: item_base.display_kor, item_type_name: item_type.name.clone() }
}