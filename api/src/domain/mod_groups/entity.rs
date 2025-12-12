use std::collections::HashSet;

use crate::domain::mods::Mod;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModGroup{
    pub id: i32,
    pub value: String,
    pub mod_generation_type: ModGenerationType,
    pub item_type: ItemType,
    pub tags: HashSet<Tag>,
    pub families: HashSet<Family>,
    pub section: Section,
    pub mods: Vec<Mod>,
}

impl ModGroup {
    pub fn new(id: i32, value: String, mod_generation_type: ModGenerationType, item_type: ItemType, tags: HashSet<Tag>, families: HashSet<Family>, section: Section, mods: Vec<Mod>) 
    -> Self {
        ModGroup { id, value, mod_generation_type, item_type, tags, families, section, mods }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModGenerationType{
    pub id: i32,
    pub label: Option<String>
}

impl ModGenerationType {
    pub fn new(id: i32, label: Option<String>) -> Self {
        ModGenerationType { id, label }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Section{
    pub id: i32,
    pub name: String,
}

impl Section {
    pub fn new(id: i32, name: String) -> Self {
        Section { id, name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemType{
    pub id: i32,
    pub name: String,
}

impl ItemType {
    pub fn new(id: i32, name: String) -> Self {
        ItemType { id, name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag{
    pub id: i32,
    pub name: String,
}

impl Tag {
    pub fn new(id: i32, name: String) -> Self {
        Tag { id, name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Family{
    pub id: i32,
    pub name: String,
}

impl Family {
    pub fn new(id: i32, name: String) -> Self {
        Family { id, name }
    }
}