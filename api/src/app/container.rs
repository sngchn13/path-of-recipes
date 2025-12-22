use std::sync::Arc;

use sea_orm::DatabaseConnection;
use crate::domain::images::{ImageRepository, ImageRepositoryImpl, ImageService, ImageServiceImpl};
use crate::domain::item_types::{ItemTypeRepository, ItemTypeRepositoryImpl};
use crate::domain::mod_groups::{ModGroupRepository, ModGroupRepositoryImpl, ModGroupService, ModGroupServiceImpl};
use crate::domain::item_bases::{ItemBaseRepository, ItemBaseRepositoryImpl, ItemBaseService, ItemBaseServiceImpl};

#[derive(Clone)]
pub struct Services {
    pub mod_group_service: Arc<dyn ModGroupService + Send + Sync>,
    pub item_base_service: Arc<dyn ItemBaseService + Send + Sync>,
    pub image_service: Arc<dyn ImageService + Send + Sync>,
}

impl Services {
    pub fn from(conn: DatabaseConnection) -> Self {

        let mod_group_repository = Arc::new(ModGroupRepositoryImpl::new(conn.clone())) as Arc<dyn ModGroupRepository>;
        let item_type_repository = Arc::new(ItemTypeRepositoryImpl::new(conn.clone())) as Arc<dyn ItemTypeRepository>;
        let item_base_repository = Arc::new(ItemBaseRepositoryImpl::new(conn.clone())) as Arc<dyn ItemBaseRepository>;
        let image_repository = Arc::new(ImageRepositoryImpl::new(conn.clone())) as Arc<dyn ImageRepository>;

        Self { 
            mod_group_service:  Arc::new(ModGroupServiceImpl::new(Arc::clone(&mod_group_repository), Arc::clone(&item_type_repository))),
            item_base_service: Arc::new(ItemBaseServiceImpl::new(Arc::clone(&item_base_repository), Arc::clone(&item_type_repository))),
            image_service: Arc::new(ImageServiceImpl::new(Arc::clone(&image_repository))),
        }
    }
}