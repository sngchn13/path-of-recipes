use std::collections::HashMap;

use entity::{item_bases, item_types, node_mods, nodes};
use sea_orm::{ActiveValue::Set, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, ColumnTrait};

use crate::{domain::{graphs::Graph, item_bases::ItemBase, item_types::ItemType, nodes::{Node, error::NodeRepositoryError}, posts::NewNode}, shared::to_node};

mod dto;
pub use dto::*;

pub async fn save_nodes_returning_map(new_nodes: Vec<NewNode>, graph: &Graph,txn: &DatabaseTransaction) -> Result<HashMap<String, Node>, NodeRepositoryError>{
    
    let mut result = HashMap::new();

    for new_node in new_nodes {
        let map_id = new_node.id;

        let model = nodes::ActiveModel{
            context: Set(new_node.context),
            rarity: Set(new_node.rarity.to()),
            item_level: Set(new_node.item_level),
            note: Set(new_node.note),
            position_x: Set(new_node.position_x),
            position_y: Set(new_node.position_y),
            item_base_id: Set(new_node.item_base_id),
            graph_id: Set(graph.id),
            ..Default::default()
        };

        let model = nodes::Entity::insert(model).exec_with_returning(txn).await?;

        let mut node_mods = Vec::new();
        for mod_with_placement in new_node.mods {
            let mod_id = mod_with_placement.mod_id;
            let placement = mod_with_placement.placement;
    
            let node_mod = node_mods::ActiveModel{
                node_id: Set(model.id),
                mod_id: Set(mod_id),
                placement: Set(placement.to())
            };

            node_mods.push(node_mod);
        }
        node_mods::Entity::insert_many(node_mods).exec(txn).await?;
        
        result.insert(map_id, to_node(model));
    }

    Ok(result)
}

pub async fn find_nodes_by_graph_id(graph_id: &i32, conn: &DatabaseConnection) -> Result<Vec<Node>, NodeRepositoryError>{
    
    let nodes: Vec<Node> = nodes::Entity::find().filter(nodes::Column::GraphId.eq(*graph_id)).all(conn).await?
        .into_iter()
        .map(|orm| to_node(orm))
        .collect();
    
    Ok(nodes)
}

pub async fn find_nodes_with_related_by_graph_id(graph_id: &i32, conn: &DatabaseConnection) 
-> Result<Vec<RawNodeAggregate>, NodeRepositoryError>
{
    
    let rows = nodes::Entity::find()
        .filter(nodes::Column::GraphId.eq(*graph_id))
        .find_also(nodes::Entity, item_bases::Entity)
        .find_also(item_bases::Entity, item_types::Entity)
        .all(conn)
        .await?;

    let mut result = Vec::new();
    for (node, item_base, item_type) in rows {
        let node = to_node(node);
        let item_base = item_base
            .and_then(|orm| Some(ItemBase::new(orm.id, orm.name, orm.display_en, orm.display_kor, orm.item_type_id)))
            .ok_or(NodeRepositoryError::ItemBaseNotFound(node.id.to_string()))?;
        let item_type = item_type
            .and_then(|orm| Some(ItemType::new(orm.id, orm.name)))
            .ok_or(NodeRepositoryError::ItemTypeNotFound(item_base.id.to_string()))?;

        result.push(RawNodeAggregate::assembly(node, item_base, item_type));
    }

    Ok(result)
}