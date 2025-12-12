use std::sync::Arc;

use sea_orm::DatabaseConnection;
use crate::domain::mod_groups::{ModGroupRepositoryImpl, ModGroupService, ModGroupServiceImpl};

#[derive(Clone)]
pub struct Services {
    pub mod_group_service: Arc<dyn ModGroupService>
}

impl Services {
    pub fn from(conn: DatabaseConnection) -> Self {
        Self { 
            mod_group_service:  Arc::new(ModGroupServiceImpl::new(Arc::new(ModGroupRepositoryImpl::new(conn))))
        }
    }
}