use std::{env, fs, path::PathBuf};

use entity::causes;
use sea_orm_migration::{ prelude::*, sea_orm::EntityTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let seed_data_directory = env::var("SEED_DATA_DIRECTORY").expect("SEED_DATA_DIRECTORY is not set in .env file");
        let cause_data_directory = PathBuf::from(&seed_data_directory).join("cause");
        let cause_data_directory = match fs::read_dir(&cause_data_directory)  {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return Ok(());
            },
        };

        let paths: Vec<PathBuf> = cause_data_directory
            .filter_map(|result| result.ok())
            .filter_map(|entry| {
                let path = entry.path();
                match path.extension().and_then(|ext| ext.to_str()) {
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
        causes::Entity::delete_many()
            .exec(db)
            .await?;

        Ok(())
    }
}