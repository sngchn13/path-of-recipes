use entity::{item_bases};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use crate::{domain::{item_bases::{ItemBaseRepositoryError, entity::ItemBase, repository::ItemBaseRepository}, item_types::ItemType}, shared::to_item_base};


pub struct ItemBaseRepositoryImpl{
    database_connection: DatabaseConnection,
}

impl ItemBaseRepositoryImpl {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self { database_connection }
    }
}

#[async_trait::async_trait]
impl ItemBaseRepository for ItemBaseRepositoryImpl {
    async fn find_all_by_item_type(&self, item_type: &ItemType) -> Result<Vec<ItemBase>, ItemBaseRepositoryError>{

        //아이템 베이스 가져오기
        let item_base_list: Vec<item_bases::Model> = item_bases::Entity::find()
            .filter(item_bases::Column::ItemTypeId.eq(item_type.id))
            .order_by_asc(item_bases::Column::Id)
            .all(&self.database_connection)
            .await
            .map_err(ItemBaseRepositoryError::Db)?;

        if item_base_list.is_empty(){ return Ok(Vec::new()); }

        //조립
        let item_base_list: Vec<ItemBase> = item_base_list
            .iter()
            .map(|ib| to_item_base(ib, &item_type))
            .collect();

        Ok(item_base_list)
    }
}