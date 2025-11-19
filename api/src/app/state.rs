use axum::{extract::FromRef};
use sea_orm::DatabaseConnection;

use crate::infrastructure::get_connection;

#[derive(Clone, FromRef)]
pub struct AppState{
    pub conn: DatabaseConnection,
}

impl AppState {
    fn new(conn: DatabaseConnection) -> Self{
        AppState {
            conn
        }
    }
}

pub async fn get_state() -> AppState {
    AppState::new(get_connection().await)
}