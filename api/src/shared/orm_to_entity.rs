
use std::collections::HashSet;

use entity::{ causes::Model as CauseOrm, edges::Model as EdgeOrm, graphs::Model as GraphOrm, images::Model as ImageOrm , item_bases::Model as ItemBaseOrm, item_types::Model as ItemTypeOrm, 
    families::Model as FamilyOrm, mod_generation_types::Model as ModGenerationTypeOrm, mods::Model as ModOrm, sections::Model as SectionOrm, tags::Model as TagOrm,
    mod_groups::Model as ModGroupOrm, node_mods::Model as NodeModOrm, nodes::Model as NodeOrm, posts::Model as PostOrm 
};

use crate::{domain::{causes::Cause, edges::Edge, graphs::{Graph, Verification}, images::Image, item_bases::ItemBase, item_types::ItemType, mod_groups::{Family, ModGenerationType, ModGroup, Section, Tag}, mods::Mod, node_mod::{NodeMod, Placement}, nodes::{Flags, Influence, Node, Rarity}, posts::Post}, shared::{fixed_offset_opt_to_offset_opt, fixed_offset_to_offset_datetime}};

pub fn to_cause(orm: CauseOrm)-> Cause {
    Cause::new(orm.id, orm.name, orm.cause_type_id)
}

pub fn to_edge(orm: EdgeOrm) -> Edge {
    Edge::new(
        orm.id,
        orm.note,
        orm.source_id,
        orm.target_id,
        orm.graph_id,
    )
}

pub fn to_graph(orm: GraphOrm) -> Graph {
    Graph::new(
        orm.id,
        orm.description,
        Verification::from(orm.verification),
        orm.post_id,
    )
}

pub fn to_image(orm: ImageOrm) -> Image{
    Image { id: orm.id, relative_path: orm.relative_path, name: orm.name, width: orm.width, height: orm.height, size: orm.size, extension: orm.extension, desc: orm.desc }
}

pub fn to_item_base(item_base_model: &ItemBaseOrm, item_type: &ItemType)
-> ItemBase {
    ItemBase::new(
        item_base_model.id, 
        item_base_model.name.clone(), 
        item_base_model.display_en.clone(), 
        item_base_model.display_kor.clone(), 
        item_type.id,
    )
}

pub fn to_item_type(orm: ItemTypeOrm) -> ItemType{
    ItemType::new(orm.id, orm.name)
}

pub fn to_mod_group(
    orm: ModGroupOrm,
    mod_generation_type: ModGenerationType,
    item_type: ItemType,
    tags: HashSet<Tag>,
    families: HashSet<Family>,
    section: Section,
    mods: Vec<Mod>,
) -> ModGroup {
    ModGroup::new(
        orm.id,
        orm.value,
        mod_generation_type,
        item_type,
        tags,
        families,
        section,
        mods,
    )
}

pub fn to_mod_generation_type(orm: &ModGenerationTypeOrm) -> ModGenerationType {
    ModGenerationType::new(orm.id, orm.label.clone())
}

pub fn to_section(orm: &SectionOrm) -> Section {
    Section::new(orm.id, orm.name.clone())
}

pub fn to_tag(orm: &TagOrm) -> Tag {
    Tag::new(orm.id, orm.name.clone())
}

pub fn to_family(orm: &FamilyOrm) -> Family {
    Family::new(orm.id, orm.name.clone())
}

pub fn to_mod(orm: &ModOrm) -> Mod {
    let created_at = fixed_offset_to_offset_datetime(orm.created_at);
    let updated_at = fixed_offset_opt_to_offset_opt(orm.updated_at);

    Mod::new(
        orm.id,
        orm.name_en.clone(),
        orm.name_kor.clone(),
        orm.display_en.clone(),
        orm.display_kor.clone(),
        orm.req_item_level,
        orm.weight,
        orm.tier,
        orm.deprecated_version.clone(),
        created_at,
        updated_at,
    )
}

pub fn to_node_mod(orm: NodeModOrm) -> NodeMod {
    NodeMod { node_id: orm.node_id, mod_id: orm.mod_id, placement: Placement::from(orm.placement), fractured: orm.fractured }
}

pub fn to_node(orm: NodeOrm)
    -> Node {
    Node { 
        id: orm.id, 
        context: serde_json::from_str(&orm.context).unwrap_or_default(), 
        rarity: Rarity::from(orm.rarity), 
        item_level: orm.item_level, 
        quality: orm.quality,
        flags: Flags::from_bits(orm.flags),
        influences: Influence::from_bits(orm.influences), 
        note: orm.note, 
        position_x: orm.position_x, 
        position_y: orm.position_y, 
        item_base_id: orm.item_base_id,

    }
}

pub fn to_post(orm: PostOrm)-> Post {
    Post::new(
        orm.id,
        orm.title,
        orm.content,
        orm.password,
        orm.view_count,
        orm.like_count,
        orm.dislike_count,
        fixed_offset_to_offset_datetime(orm.created_at),
        fixed_offset_opt_to_offset_opt(orm.updated_at)
    )
}
