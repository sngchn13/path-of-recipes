use std::{fmt, str::FromStr};

use migration::{Migrator, MigratorTrait};
use sea_orm::{DatabaseConnection, DbErr};
use tracing::info;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum MigrateLevel {
    Up,
    Down,
    Fresh,
}

impl FromStr for MigrateLevel {
    type Err = MigrateLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_ascii_lowercase();

        match s.as_str() {
            "up" => Ok(MigrateLevel::Up),
            "down" => Ok(MigrateLevel::Down),
            "fresh" => Ok(MigrateLevel::Fresh),
            _ => Err(MigrateLevelError::Parse(s)),
        }
    }
}

impl fmt::Display for MigrateLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MigrateLevel::Up => "up",
            MigrateLevel::Down => "down",
            MigrateLevel::Fresh => "fresh",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Error)]
enum MigrateLevelError{
    #[error("parsing fail = {0}")]
    Parse(String)
}

impl MigrateLevel {
    pub fn from_env() -> Option<Self> {
        let raw = std::env::var("MIGRATION_LEVEL").ok()?;
        match raw.parse() {
            Ok(level) => Some(level),
            Err(e) => {
                tracing::warn!("invalid MIGRATION_LEVEL: {e}");
                None
            }
        }
    }

    pub async fn migrate(self, conn: &DatabaseConnection) -> Result<(), DbErr> {
        match self {
            MigrateLevel::Up => Migrator::up(conn, None).await,
            MigrateLevel::Down => Migrator::down(conn, None).await,
            MigrateLevel::Fresh => Migrator::fresh(conn).await,
        }
    }
}


pub async fn run_migration_from_env(conn: &DatabaseConnection) -> Result<(), DbErr> {
    if let Some(level) = MigrateLevel::from_env() {
        level.migrate(conn).await?;
        info!("migration complete migration level={}", level);
    }
    Ok(())
}
