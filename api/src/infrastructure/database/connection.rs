use std::{env, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn get_connection() -> DatabaseConnection{
    let mut opt = ConnectOptions::new(env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file"));
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    match Database::connect(opt).await {
        Ok(db) => db,
        Err(e) => panic!("Error connection to database: {}", e),
    }
}