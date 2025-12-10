use std::{collections::{HashMap, HashSet}, env, fs::{self, DirEntry, File}, io::Read, path::PathBuf};

use entity::{families, item_types, mod_group_families, mod_group_tags, mod_groups, mods, sections, tags};
use sea_orm_migration::{prelude::*};
use serde::Deserialize;
use anyhow::{Result, anyhow};
use sea_orm::{EntityTrait, Set};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        let resources_directory = env::var("RESOURCES_DIRECTORY").expect("RESOURCES_DIRECTORY is not set in .env file");
        let mod_directory = fs::read_dir(&PathBuf::from(resources_directory).join("csv").join("mod")).expect("mod diretory doesn't exist");
    
        let cache = PropertyCache::new(
            make_item_type_list(db).await.expect("item type generation error"), 
            make_section_list(db).await.expect("section generation error"), 
            make_tag_list(db).await.expect("tag generation error"), 
            make_family_list(db).await.expect("family generation error"),
        );

        for dir_entry_result  in mod_directory {
            let dir_entry = dir_entry_result
                .map_err(|e| DbErr::Custom(format!("read_dir error: {e}")))?;

            let file_type = dir_entry.file_type()
                .map_err(|e| DbErr::Custom(format!("file_type error: {e}")))?;
            if !file_type.is_dir() {
                continue;
            }
            
            if let Err(e)= process_directory(dir_entry, &cache, db).await{
                return Err(DbErr::Custom(e.to_string()));
            }
        }
        
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
    
}

async fn process_directory<'c>(dir: DirEntry,cache: &PropertyCache, db: &SchemaManagerConnection<'c>) 
-> Result<()>{
    
    let directory = fs::read_dir(&dir.path())?;

    let item_type_name = dir.file_name().to_string_lossy().into_owned();
    
    for entry_result in directory {
        let entry = entry_result?;
        process_file(&item_type_name, entry, cache, db).await?;
    }

    Ok(())
}

async fn process_file<'c>(item_type_name: &str, entry: DirEntry, cache: &PropertyCache, db: &SchemaManagerConnection<'c>) 
-> Result<()>{
    let file_path = entry.path();
    let section_name = file_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let file = match File::open(&file_path) {
        Ok(f) => f,
        Err(_) => { return Ok(()); }
    };

    let csv_records = read_csv_from_reader(file);
    let csv_mod_groups = section_record_transform(csv_records, &item_type_name, &section_name);
    persist_mods_per_mod_group(csv_mod_groups, &cache, db).await?;
    Ok(())
}

fn read_csv_from_reader<R: Read>(reader: R) -> Vec<CsvRecord> {
    let mut tmp = Vec::new();
    let mut reader = csv::Reader::from_reader(reader);

    for result in reader.deserialize() {
        if let Ok(section_record) = result {
            tmp.push(section_record);
        }
    }

    tmp
}

fn section_record_transform(csv_records: Vec<CsvRecord>, item_type_name: &str, section_name: &str) -> Vec<CsvModGroup>{
    let mut tmp: HashMap<String, CsvModGroup> = HashMap::new();

    //섹션 레코드를 모드 그룹으로 바꿔서 벡터에 저장하기
    for record in csv_records {
        let mod_item = CsvMod::new(record.name_en, record.name_kor, record.str_en,record.str_kor,  record.level, record.weight);

        match tmp.get_mut(&record.mod_group) {
            Some(mod_group) => {
                mod_group.mod_list.push(mod_item);
            },
            None => {

                // family_set
                let family_set: HashSet<String> = match record.mod_family_list {
                    Some(str) => {
                        str.split('|')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect()
                    },
                    None => HashSet::new(),
                }; 

                // tag_set
                let tag_set: HashSet<String> = match record.tags {
                    Some(str) => {
                        str.split('|')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect()
                    },
                    None => HashSet::new(),
                };


                let mod_group = CsvModGroup::new(
                    record.mod_group.to_string(), 
                    record.mod_generation_type_id, 
                    family_set, 
                    tag_set,
                    vec![mod_item], 
                    item_type_name.to_string(), 
                    section_name.to_string(),
                );

                tmp.insert(record.mod_group, mod_group);
            }
        };
    }

    tmp.into_values().collect()
}

async fn persist_mods_per_mod_group<'c>(csv_mod_groups: Vec<CsvModGroup>, cache: &PropertyCache, db: &SchemaManagerConnection<'c>)
    -> Result<()>
{

    let mut mod_group_tag_items = Vec::new();
    let mut mod_group_family_items = Vec::new();
    let mut mod_items = Vec::new();

    for csv_mod_group in csv_mod_groups {
    
        let item_type_id = cache.get_item_type_id(&csv_mod_group.item_type)
            .ok_or_else(|| anyhow!("item type({}) doesn't exists", &csv_mod_group.item_type))?;
        let section_id = cache.get_section_id(&csv_mod_group.section)
            .ok_or_else(|| anyhow!("section({}) doesn't exists", &csv_mod_group.section))?;

        //mod_group 저장
        let mod_group_model = mod_groups::ActiveModel{
            value: Set(csv_mod_group.value),
            mod_generation_type_id: Set(csv_mod_group.generation_type_id),
            item_type_id: Set(item_type_id),
            section_id: Set(section_id),
            ..Default::default()
        };
        let saved_mod_group_model = mod_groups::Entity::insert(mod_group_model).exec_with_returning(db).await?;
        
        //태그 저장
        for tag_name in csv_mod_group.tag_set {
            let tag_id = cache.get_tag_id(&tag_name)
                .ok_or_else(|| anyhow!("tag({}) doesn't exists", &tag_name))?;
            
            let mod_tag_item = mod_group_tags::ActiveModel{
                mod_group_id: Set(saved_mod_group_model.id),
                tag_id: Set(tag_id),
            };
            //dbg!((&saved_mod_group_model.id, &tag_id));

            mod_group_tag_items.push(mod_tag_item);
        }
        
        //패밀리 저장
        for family_name in csv_mod_group.family_set {
            let family_id = cache.get_family_id(&family_name)
                .ok_or_else(|| anyhow!("family({}) doesn't exist", family_name))?;

            let mod_group_family_item = mod_group_families::ActiveModel{
                mod_group_id: Set(saved_mod_group_model.id),
                family_id: Set(family_id),
            };
            
            mod_group_family_items.push(mod_group_family_item);
        }

        //mod 저장
        for (idx, csv_mod) in csv_mod_group.mod_list.into_iter().enumerate() {
            let mod_item = mods::ActiveModel{
                name_en: Set(csv_mod.name_en),
                name_kor: Set(csv_mod.name_kor),
                display_en: Set(csv_mod.display_en),
                display_kor: Set(csv_mod.display_kor),
                req_item_level: Set(csv_mod.level),
                weight: Set(csv_mod.weight), //weight값 가져오기,
                tier: Set(Some((idx + 1) as i16)),
                mod_group_id: Set(saved_mod_group_model.id),
                deprecated_version: Set(None),
                ..Default::default()
            };
            mod_items.push(mod_item);
        }
    }

    if !mod_group_tag_items.is_empty(){
        mod_group_tags::Entity::insert_many(mod_group_tag_items).exec(db).await?;
    }
    if !mod_group_family_items.is_empty(){
        mod_group_families::Entity::insert_many(mod_group_family_items).exec(db).await?;
    }
    if !mod_items.is_empty(){
        mods::Entity::insert_many(mod_items).exec(db).await?;
    }

    Ok(())
}

async fn make_item_type_list<'c>(db: &SchemaManagerConnection<'c>) -> Result<HashMap<String, i32>>{
    let mut res = HashMap::new();
    let item_type_models = item_types::Entity::find().all(db).await?;
    
    for item_type_model in item_type_models {
        res.insert(item_type_model.name, item_type_model.id);
    }

    Ok(res)
}

async fn make_section_list<'c>(db: &SchemaManagerConnection<'c>) -> Result<HashMap<String, i32>>{
    let mut res = HashMap::new();
    let section_models = sections::Entity::find().all(db).await?;
    
    for section_model in section_models {
        res.insert(section_model.name, section_model.id);
    }

    Ok(res)
}

async fn make_tag_list<'c>(db: &SchemaManagerConnection<'c>) -> Result<HashMap<String, i32>>{
    let mut res = HashMap::new();
    let tags = tags::Entity::find().all(db).await?;
    
    for tag in tags {
        res.insert(tag.name, tag.id);
    }

    Ok(res)
}

async fn make_family_list<'c>(db: &SchemaManagerConnection<'c>) -> Result<HashMap<String, i32>>{
    let mut res = HashMap::new();
    let families = families::Entity::find().all(db).await?;
    
    for family in families {
        res.insert(family.name, family.id);
    }

    Ok(res)
}

#[derive(Debug, Deserialize)]
struct CsvRecord {
    #[serde(rename = "EnName")]
    name_en: String,
    #[serde(rename = "KorName")]
    name_kor: String,
    #[serde(rename = "Level")]
    level: i32,
    #[serde(rename = "DropChance")]
    weight: Option<i16>,
    #[serde(rename = "ModGenerationTypeId")]
    mod_generation_type_id: i32,
    #[serde(rename = "ModFamilyList")]
    mod_family_list: Option<String>,
    #[serde(rename = "StrEn")]
    str_en: String,
    #[serde(rename = "StrKor")]
    str_kor: String,
    #[serde(rename = "fossil_no")]
    tags: Option<String>,
    #[serde(rename = "ModGroup")]
    mod_group: String,
}

struct CsvModGroup{
    value: String,
    generation_type_id: i32,
    family_set: HashSet<String>,
    tag_set: HashSet<String>,
    mod_list: Vec<CsvMod>,
    item_type: String,
    section: String,
}

impl CsvModGroup {
    fn new(value: String, generation_type_id: i32, family_set: HashSet<String>, tag_set: HashSet<String>, mod_list: Vec<CsvMod>, item_type: String, section: String)
     -> CsvModGroup 
    {
        CsvModGroup { value, generation_type_id, family_set, tag_set, mod_list, item_type, section }
    }
}

struct CsvMod{
    name_en: String,
    name_kor: String,
    display_en: String,
    display_kor: String,
    level: i32,
    weight: Option<i16>,
}

impl CsvMod {
    fn new(name_en: String, name_kor: String, display_en: String, display_kor: String, level: i32, weight: Option<i16>) -> CsvMod {
        CsvMod { name_en, name_kor, display_en, display_kor, level, weight }
    }
}

struct PropertyCache{
    item_type_map: HashMap<String, i32>,
    section_map: HashMap<String, i32>,
    tag_map: HashMap<String, i32>,
    family_map: HashMap<String, i32>
}

impl PropertyCache {
    fn new(item_type_map: HashMap<String, i32>, section_map: HashMap<String, i32>, tag_map: HashMap<String, i32>, family_map: HashMap<String, i32>) 
    -> Self
    {
        Self { item_type_map, section_map, tag_map, family_map }
    }

    fn get_item_type_id(&self, name: &str)-> Option<i32>{
        self.item_type_map.get(name).copied()
    }

    fn get_section_id(&self, name: &str)-> Option<i32>{
        self.section_map.get(name).copied()
    }

    fn get_tag_id(&self, name: &str)-> Option<i32>{
        self.tag_map.get(name).copied()
    }

    fn get_family_id(&self, name: &str)-> Option<i32>{
        self.family_map.get(name).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const CSV_DATA: &str = r#"EnName,KorName,Level,ModGenerationTypeId,ModFamilyList,StrEn,StrKor,StatsStr,DropChance,fossil_no,mod_no,mod_fossil_item,ModGroup
of the Brute,- 짐승,1,2,Strength,+(8—12) to Strength,힘 +(8-12),,1000,attribute,Attribute,,+# to Strength
of the Wrestler,- 씨름꾼,11,2,Strength,+(13—17) to Strength,힘 +(13-17),,1000,attribute,Attribute,,+# to Strength
of the Bear,- 곰,22,2,Strength,+(18—22) to Strength,힘 +(18-22),,1000,attribute,Attribute,,+# to Strength
of the Mongoose,- 몽구스,1,2,Dexterity,+(8—12) to Dexterity,민첩 +(8-12),,1000,attribute,Attribute,,+# to Dexterity
of the Lynx,- 스라소니,11,2,Dexterity,+(13—17) to Dexterity,민첩 +(13-17),,1000,attribute,Attribute,,+# to Dexterity
of the Fox,- 여우,22,2,Dexterity,+(18—22) to Dexterity,민첩 +(18-22),,1000,attribute,Attribute,,+# to Dexterity
of the Pupil,- 제자,1,2,Intelligence,+(8—12) to Intelligence,지능 +(8-12),,1000,attribute,Attribute,,+# to Intelligence
of the Student,- 학생,11,2,Intelligence,+(13—17) to Intelligence,지능 +(13-17),,1000,attribute,Attribute,,+# to Intelligence
of the Prodigy,- 영재,22,2,Intelligence,+(18—22) to Intelligence,지능 +(18-22),,1000,attribute,Attribute,,+# to Intelligence
"#;

    #[test]
    fn read_csv_from_reader_parses_all_rows() {
        let records = read_csv_from_reader(CSV_DATA.as_bytes());

        // 1) 레코드 개수 확인
        assert_eq!(records.len(), 9);

        // 2) 첫 번째 레코드 확인
        let r0 = &records[0];
        assert_eq!(r0.name_en, "of the Brute");
        assert_eq!(r0.name_kor, "- 짐승");
        assert_eq!(r0.level, 1);
        assert_eq!(r0.mod_generation_type_id, 2);
        assert_eq!(r0.mod_family_list.as_deref(), Some("Strength"));
        assert_eq!(r0.str_en, "+(8—12) to Strength");
        assert_eq!(r0.str_kor, "힘 +(8-12)");
        assert_eq!(r0.tags.as_deref(), Some("attribute"));
        assert_eq!(r0.mod_group, "+# to Strength");
        assert_eq!(r0.weight.unwrap_or_else(|| panic!("weight is None!")), 1000);

        // 3) 마지막 레코드도 확인
        let r8 = &records[8];
        assert_eq!(r8.name_en, "of the Prodigy");
        assert_eq!(r8.name_kor, "- 영재");
        assert_eq!(r8.level, 22);
        assert_eq!(r8.mod_family_list.as_deref(), Some("Intelligence"));
        assert_eq!(r8.tags.as_deref(), Some("attribute"));
        assert_eq!(r8.mod_group, "+# to Intelligence");
        assert_eq!(r8.weight.unwrap_or_else(|| panic!("weight is None!")), 1000);
    }

    #[test]
    fn section_record_transform_groups_records_by_mod_group() {
        let records = read_csv_from_reader(CSV_DATA.as_bytes());
        assert_eq!(records.len(), 9);

        let item_type_name = "attribute";
        let section_name = "normal";
        let groups = section_record_transform(records, item_type_name, section_name);

        assert_eq!(groups.len(), 3);

        // Strength 그룹 ("+# to Strength")
        let strength_group = groups
            .iter()
            .find(|g| g.value == "+# to Strength")
            .expect("group '+# to Strength' not found");

        assert_eq!(strength_group.generation_type_id, 2);
        assert_eq!(
            strength_group.family_set,
            HashSet::from(["Strength".to_string()])
        );
        assert_eq!(
            strength_group.tag_set,
            HashSet::from(["attribute".to_string()])
        );
        assert_eq!(strength_group.item_type, item_type_name.to_string());
        assert_eq!(strength_group.section, section_name.to_string());
        // Brute / Wrestler / Bear 3개
        assert_eq!(strength_group.mod_list.len(), 3);

        let m0 = &strength_group.mod_list[0];
        assert_eq!(m0.name_en, "of the Brute");
        assert_eq!(m0.name_kor, "- 짐승");
        assert_eq!(m0.display_en, "+(8—12) to Strength");
        assert_eq!(m0.display_kor, "힘 +(8-12)");
        assert_eq!(m0.level, 1);

        // Intelligence 그룹 ("+# to Intelligence") 샘플 체크
        let int_group = groups
            .iter()
            .find(|g| g.value == "+# to Intelligence")
            .expect("group '+# to Intelligence' not found");

        assert_eq!(
            int_group.family_set,
            HashSet::from(["Intelligence".to_string()])
        );
        assert_eq!(int_group.mod_list.len(), 3);
        let last = int_group
            .mod_list
            .iter()
            .find(|m| m.name_en == "of the Prodigy")
            .expect("mod 'of the Prodigy' not found");
        assert_eq!(last.level, 22);
        assert_eq!(last.display_kor, "지능 +(18-22)");
    }
}
