use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::{domain::item_types::{ItemType, error::ItemTypeRepositoryError, repository::ItemTypeRepository}, shared::to_item_type};
use entity::{item_types};


pub struct ItemTypeRepositoryImpl{
    database_connection: DatabaseConnection,
}

impl ItemTypeRepositoryImpl {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self { database_connection }
    }
}

#[async_trait::async_trait]
impl ItemTypeRepository for ItemTypeRepositoryImpl {
    async fn find_by_name(&self, name: &str) 
    -> Result<Option<ItemType>, ItemTypeRepositoryError>
    {
        let item_type = item_types::Entity::find()
            .filter(item_types::Column::Name.eq(name))
            .one(&self.database_connection)
            .await
            .map_err(ItemTypeRepositoryError::Db)?;

         Ok(item_type.map(to_item_type))
    }
}