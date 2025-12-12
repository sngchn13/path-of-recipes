use std::collections::HashSet;

use entity::{families, item_types, mod_generation_types, mod_group_families, mod_group_tags, mod_groups, mods, sections, tags};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter, QueryOrder};

use crate::domain::mod_groups::{ModGroup, error::ModGroupRepositoryError, repository::ModGroupRepository};

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
    async fn find_all_by_item_type(&self, item_type_name: &str) -> Result<Vec<ModGroup>, super::ModGroupRepositoryError> {
        //아이템 타입
        let item_type = item_types::Entity::find()
            .filter(item_types::Column::Name.eq(item_type_name))
            .one(&self.database_connection)
            .await
            .map_err(ModGroupRepositoryError::Db)?
            .ok_or_else(||ModGroupRepositoryError::ItemTypeNotFound(item_type_name.to_string()))?;

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
        let item_type = orm_to_domain::to_item_type(&item_type);
        let mut result = Vec::with_capacity(mod_group_list.len());
        for (idx, mod_group) in mod_group_list.into_iter().enumerate() {
            let mod_generation_type = mod_generation_type_list
                .get(idx)
                .and_then(|opt| opt.as_ref())                                     
                .map(orm_to_domain::to_mod_generation_type)       
                .ok_or_else(|| ModGroupRepositoryError::AssembleError(format!("mod generation type id({}) is empty", &mod_group.mod_generation_type_id)))?;

            let section = section_list
                .get(idx)
                .and_then(|opt| opt.as_ref())
                .map(orm_to_domain::to_section)
                .ok_or_else(|| ModGroupRepositoryError::AssembleError(format!("section({}) is empty", &mod_group.section_id)))?;

            let families = families_list
                .get(idx)
                .map(|fam_vec| {
                    fam_vec
                        .iter()
                        .map(orm_to_domain::to_family)
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
                        .map(orm_to_domain::to_tag)
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
                        .map(orm_to_domain::to_mod)
                        .collect::<Vec<_>>()
                })
                .ok_or_else(|| ModGroupRepositoryError::AssembleError(
                    format!("mods for mod_group_id({}) is empty", &mod_group.id)
                ))?;
            
            let mod_group = orm_to_domain::to_mod_group(mod_group, mod_generation_type, item_type.clone(), tags, families, section, mods);

            result.push(mod_group);
        }

        Ok(result)
    }
}



mod orm_to_domain {
    use std::collections::HashSet;

    use chrono::{DateTime, FixedOffset, Utc};
    use time::OffsetDateTime;

    use entity::{
        families::Model as FamilyOrm,
        item_types::Model as ItemTypeOrm,
        mod_generation_types::Model as ModGenerationTypeOrm,
        mods::Model as ModOrm,
        sections::Model as SectionOrm,
        tags::Model as TagOrm,
        mod_groups::Model as ModGroupOrm,
    };

    use crate::domain::{
        mod_groups::entity::*,
        mods::entity::Mod,
    };

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

    /// ORM ModGenerationType → 도메인 ModGenerationType
    pub fn to_mod_generation_type(orm: &ModGenerationTypeOrm) -> ModGenerationType {
        ModGenerationType::new(orm.id, orm.label.clone())
    }

    /// ORM Section → 도메인 Section
    pub fn to_section(orm: &SectionOrm) -> Section {
        Section::new(orm.id, orm.name.clone())
    }

    /// ORM ItemType → 도메인 ItemType
    pub fn to_item_type(orm: &ItemTypeOrm) -> ItemType {
        ItemType::new(orm.id, orm.name.clone())
    }

    /// ORM Tag → 도메인 Tag
    pub fn to_tag(orm: &TagOrm) -> Tag {
        Tag::new(orm.id, orm.name.clone())
    }

    /// ORM Family → 도메인 Family
    pub fn to_family(orm: &FamilyOrm) -> Family {
        Family::new(orm.id, orm.name.clone())
    }

    /// ORM Mod → 도메인 Mod
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


    /// chrono::DateTime<FixedOffset> → time::OffsetDateTime
    fn fixed_offset_to_offset_datetime(dt: DateTime<FixedOffset>) -> OffsetDateTime {
        // DB에 FixedOffset으로 들어왔어도, 도메인에서는 UTC 기준 OffsetDateTime 사용
        let utc: DateTime<Utc> = dt.with_timezone(&Utc);

        OffsetDateTime::from_unix_timestamp(utc.timestamp())
            .expect("invalid unix timestamp when converting from chrono to time")
    }

    /// Option<chrono::DateTime<FixedOffset>> → Option<OffsetDateTime>
    fn fixed_offset_opt_to_offset_opt(dt: Option<DateTime<FixedOffset>>) -> Option<OffsetDateTime> {
        dt.map(fixed_offset_to_offset_datetime)
    }
}


#[cfg(test)]
mod tests {

}