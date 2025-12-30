use std::collections::HashMap;

use entity::node_mods;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use tracing::info;

use crate::{domain::node_mod::{NodeMod, NodeModRepositoryError}, shared::to_node_mod};


pub async fn find_node_mods_from_node_ids(node_ids: Vec<i32>, conn: &DatabaseConnection) -> Result<HashMap<i32, Vec<NodeMod>>, NodeModRepositoryError>{

    info!("node ids = {:#?}", &node_ids);
    let mut result: HashMap<i32, Vec<NodeMod>> = HashMap::new();
    let orms = node_mods::Entity::find().filter(node_mods::Column::NodeId.is_in(node_ids)).all(conn).await?;
    info!("2node ids = {:#?}", &orms);

    for orm in orms {
        let node_id = orm.node_id;
        let node_mod = to_node_mod(orm);

        result.entry(node_id).or_default().push(node_mod);
    }

    Ok(result)
}