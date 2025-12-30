use std::collections::{HashMap, HashSet};

use entity::{families, item_types, mod_generation_types, mod_group_families, mod_group_tags, mod_groups, mods, sections, tags};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};

use crate::{domain::{mod_groups::{Family, Tag}, mods::{ModRepositoryError}}, shared::{fixed_offset_opt_to_offset_opt, fixed_offset_to_offset_datetime, to_family, to_item_type, to_mod_generation_type, to_section, to_tag}};

mod dto;
pub use dto::*;

pub async fn find_all_mods_with_related_by_mod_ids(mod_ids: HashSet<i32> , conn: &DatabaseConnection) -> Result<HashMap<i32, UnplacedModAggregate>, ModRepositoryError>{
    let rows = mods::Entity::find()
        .filter(mods::Column::Id.is_in(mod_ids))
        .find_also(mods::Entity, mod_groups::Entity)
        .find_also(mod_groups::Entity, mod_generation_types::Entity)
        .find_also(mod_groups::Entity, sections::Entity)
        .find_also(mod_groups::Entity, item_types::Entity)
        .all(conn).await?;

    //모드 그룹 아이디 모으기
    let mut mod_group_ids = HashSet::new();
    for (_, mod_group, _, _, _) in rows.iter() {
    if let Some(g) = mod_group {
            mod_group_ids.insert(g.id.clone());
        }
    }

    //패밀리 가져오기
    let mut mod_group_id_families_map: HashMap<i32, HashSet<Family>> = HashMap::new();
    for (mg, families_orm) in mod_group_families::Entity::find()
            .find_with_related(families::Entity)
            .filter(mod_group_families::Column::ModGroupId.is_in(mod_group_ids.clone()))
            .all(conn)
            .await?
    {
        mod_group_id_families_map
            .entry(mg.mod_group_id)
            .or_default()
            .extend(families_orm.into_iter().map(|orm| to_family(&orm)));
    }

    
    //태그 가져오기
    let mut mod_group_tags_map: HashMap<i32, HashSet<Tag>> = HashMap::new();
    for (mg, tags_orm) in mod_group_tags::Entity::find()
        .find_with_related(tags::Entity)
        .filter(mod_group_tags::Column::ModGroupId.is_in(mod_group_ids))
        .all(conn)
        .await?
    {
        mod_group_tags_map
            .entry(mg.mod_group_id)
            .or_default()
            .extend(tags_orm.into_iter().map(|orm| to_tag(&orm)));
    }

    let mut result = HashMap::new();
    //합치기
    for (mod_item, mod_group, mod_generation_type, section, item_type) in rows {
        let mod_group = mod_group.ok_or(ModRepositoryError::ModGroupNotFound(mod_item.id.to_string()))?;
        let mod_generation_type = to_mod_generation_type(&mod_generation_type.ok_or(ModRepositoryError::ModGenerationTypeNotFound(mod_group.id.to_string()))?);
        let section = to_section(&section.ok_or(ModRepositoryError::SectionNotFound(mod_group.id.to_string()))?);
        let item_type = to_item_type(item_type.ok_or(ModRepositoryError::ItemTypeNotFound(mod_group.id.to_string()))?);

        let families = mod_group_id_families_map.get(&mod_group.id).ok_or(ModRepositoryError::FamiliesNotFound(mod_group.id.to_string()))?;
        let tags = mod_group_tags_map.get(&mod_group.id).ok_or(ModRepositoryError::TagsNotFound(mod_group.id.to_string()))?;

        let unplaced_mod = UnplacedModAggregate {
            id: mod_item.id,
            name_en: mod_item.name_en,
            name_kor: mod_item.name_kor,
            display_en: mod_item.display_en,
            display_kor: mod_item.display_kor,
            req_item_level: mod_item.req_item_level,
            weight: mod_item.weight,
            tier: mod_item.tier,
            deprecated_version: mod_item.deprecated_version,
            created_at: fixed_offset_to_offset_datetime(mod_item.created_at),
            updated_at: fixed_offset_opt_to_offset_opt(mod_item.updated_at),
            mod_group_id: mod_group.id,
            mod_group_value: mod_group.value,
            mod_generation_type: mod_generation_type,
            item_type: item_type,
            tags: tags.clone(),
            families: families.clone(),
            section: section,
            
        };

        result.insert(unplaced_mod.id, unplaced_mod);
    }

    Ok(result)
}