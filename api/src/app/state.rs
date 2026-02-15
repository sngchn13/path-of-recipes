use std::sync::Arc;

use axum::{extract::FromRef};
use sea_orm::DatabaseConnection;

use crate::{app::Services, domain::{images::ImageService, item_bases::ItemBaseService, mod_groups::ModGroupService, posts::PostService}, infrastructure::get_connection};

const ENV_COOKIE_SECURE: &str = "COOKIE_SECURE";

#[derive(Clone)]
pub struct AppConfig {
    pub cookie_secure: bool,
}

#[derive(Clone, FromRef)]
pub struct AppState{
    pub conn: DatabaseConnection,
    pub services: Services,
    config: AppConfig,
}

impl AppState {
    fn new(conn: DatabaseConnection, services: Services, config: AppConfig) -> Self{
        AppState {
            conn,
            services,
            config,
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

impl FromRef<AppState> for Arc<dyn PostService> {
    fn from_ref(input: &AppState) -> Self {
        input.services.post_service.clone()
    }
}

pub async fn get_state() -> AppState {
    let database_connection = get_connection().await;

    let cookie_secure = std::env::var(ENV_COOKIE_SECURE).ok().and_then(|v| v.parse::<bool>().ok()).unwrap_or(true);

    AppState::new( database_connection.clone(), Services::from(database_connection.clone()), AppConfig { cookie_secure })
}
