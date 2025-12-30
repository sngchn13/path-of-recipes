use std::collections::HashSet;

use entity::{families, mod_generation_types, mod_group_families, mod_group_tags, mod_groups, mods, sections, tags};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter, QueryOrder};

use crate::{domain::{item_types::ItemType, mod_groups::{ModGroup, error::ModGroupRepositoryError, repository::ModGroupRepository}}, shared::{to_family, to_mod, to_mod_generation_type, to_mod_group, to_section, to_tag}};

pub struct ModGroupRepositoryImpl{ 
    database_connection: DatabaseConnection,
}

impl ModGroupRepositoryImpl {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self { database_connection }
    }
}

#[async_trait::async_trait]
impl ModGroupRepository for ModGroupRepositoryImpl {
    async fn find_all_by_item_type(&self, item_type: &ItemType) -> Result<Vec<ModGroup>, super::ModGroupRepositoryError> {

        //모드 그룹
        let mod_group_list: Vec<mod_groups::Model> = mod_groups::Entity::find()
            .filter(mod_groups::Column::ItemTypeId.eq(item_type.id))
            .order_by_asc(mod_groups::Column::Id)
            .all(&self.database_connection)
            .await
            .map_err(ModGroupRepositoryError::Db)?;

        if mod_group_list.is_empty(){ return Ok(Vec::new()); }

        let mod_generation_type_list = mod_group_list.load_one(mod_generation_types::Entity, &self.database_connection).await.map_err(ModGroupRepositoryError::Db)?;
        let section_list = mod_group_list.load_one(sections::Entity, &self.database_connection).await.map_err(ModGroupRepositoryError::Db)?;
        let families_list = mod_group_list.load_many_to_many(families::Entity, mod_group_families::Entity, &self.database_connection).await.map_err(ModGroupRepositoryError::Db)?;
        let tags_list = mod_group_list.load_many_to_many(tags::Entity, mod_group_tags::Entity, &self.database_connection).await.map_err(ModGroupRepositoryError::Db)?;
        let mods_list = mod_group_list.load_many(mods::Entity, &self.database_connection).await.map_err(ModGroupRepositoryError::Db)?;

        //조립
        let mut result = Vec::with_capacity(mod_group_list.len());
        for (idx, mod_group) in mod_group_list.into_iter().enumerate() {
            let mod_generation_type = mod_generation_type_list
                .get(idx)
                .and_then(|opt| opt.as_ref())                                     
                .map(to_mod_generation_type)       
                .ok_or_else(|| ModGroupRepositoryError::AssembleError(format!("mod generation type id({}) is empty", &mod_group.mod_generation_type_id)))?;

            let section = section_list
                .get(idx)
                .and_then(|opt| opt.as_ref())
                .map(to_section)
                .ok_or_else(|| ModGroupRepositoryError::AssembleError(format!("section({}) is empty", &mod_group.section_id)))?;

            let families = families_list
                .get(idx)
                .map(|fam_vec| {
                    fam_vec
                        .iter()
                        .map(to_family)
                        .collect::<HashSet<_>>()
                })
                .ok_or_else(|| ModGroupRepositoryError::AssembleError(
                        format!("families for mod_group_id({}) is empty", &mod_group.id)
                ))?;

            let tags = tags_list
                .get(idx)
                .map(|tag_vec| {
                    tag_vec
                        .iter()
                        .map(to_tag)
                        .collect::<HashSet<_>>()
                })
                .ok_or_else(|| ModGroupRepositoryError::AssembleError(
                    format!("tags for mod_group_id({}) is empty", &mod_group.id)
                ))?;

            let mods = mods_list
                .get(idx)
                .map(|mod_vec| {
                    mod_vec
                        .iter()
                        .map(to_mod)
                        .collect::<Vec<_>>()
                })
                .ok_or_else(|| ModGroupRepositoryError::AssembleError(
                    format!("mods for mod_group_id({}) is empty", &mod_group.id)
                ))?;
            
            let mod_group = to_mod_group(mod_group, mod_generation_type, item_type.clone(), tags, families, section, mods);

            result.push(mod_group);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {

}