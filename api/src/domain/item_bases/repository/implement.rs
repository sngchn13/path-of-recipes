use entity::{item_bases};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use crate::domain::{item_bases::{ItemBaseRepositoryError, entity::ItemBase, repository::{ItemBaseRepository, implement::orm_to_domain::to_item_base}}, item_types::ItemType};


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

mod orm_to_domain {

    use entity::{ item_bases::Model as ItemBaseOrm };

    use crate::domain::{item_bases::entity::{ItemBase, ItemTypeId}, item_types::ItemType};

    pub fn to_item_base(item_base_model: &ItemBaseOrm, item_type: &ItemType)
        -> ItemBase {
            ItemBase::new(
                item_base_model.id, 
                item_base_model.name.clone(), 
                item_base_model.display_en.clone(), 
                item_base_model.display_kor.clone(), 
                ItemTypeId::new(item_type.id),
            )
        }

}