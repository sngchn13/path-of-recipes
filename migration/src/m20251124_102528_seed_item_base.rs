use std::{env, fs, path::PathBuf};

use entity::item_bases;
use sea_orm_migration::{prelude::*, sea_orm::EntityTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let resources_directory = env::var("RESOURCES_DIRECTORY").expect("RESOURCES_DIRECTORY is not set in .env file");
        let item_base_directory = PathBuf::from(&resources_directory).join("sql").join("item_base");

        let item_base_directory = match fs::read_dir(&item_base_directory)  {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return Ok(());
            },
        };

        let paths: Vec<PathBuf> = item_base_directory
            .filter_map(|entry| entry.ok()) // 1. ok는 result를 option으로 변환함. 2. filter_map은 option에 대해 값을 꺼냄
            .filter_map(|entry| {
                let path = entry.path();
                match path.extension().and_then(|ext| ext.to_str()) { //and_then은 optione(&OsStr)을 option(&str)로 바꿔줌. map의 한 형태
                    Some(ext) if ext.eq_ignore_ascii_case("sql") => Some(path),
                    _ => None,
                }
            })
            .collect();

        let db = manager.get_connection();

        for path in paths {
            let sql = fs::read_to_string(path).ok();
            if let Some(sql) = sql {
                db.execute_unprepared(&sql).await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        item_bases::Entity::delete_many()
            .exec(db)
            .await?;
        Ok(())
    }
}
