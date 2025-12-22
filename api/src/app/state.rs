use std::sync::Arc;

use axum::{extract::FromRef};
use sea_orm::DatabaseConnection;

use crate::{app::Services, domain::{images::ImageService, item_bases::ItemBaseService, mod_groups::ModGroupService}, infrastructure::get_connection};

#[derive(Clone, FromRef)]
pub struct AppState{
    pub conn: DatabaseConnection,
    pub services: Services,
}

impl AppState {
    pub fn new(conn: DatabaseConnection, services: Services) -> Self{
        AppState {
            conn,
            services,
        }
    }
}

impl FromRef<AppState> for Arc<dyn ModGroupService> {
    fn from_ref(input: &AppState) -> Self {
        input.services.mod_group_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ItemBaseService> {
    fn from_ref(input: &AppState) -> Self {
        input.services.item_base_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ImageService> {
    fn from_ref(input: &AppState) -> Self {
        input.services.image_service.clone()
    }
}

pub async fn get_state() -> AppState {
    let database_connection = get_connection().await;

    AppState::new( database_connection.clone(), Services::from(database_connection.clone()))
}
