use std::collections::HashMap;

use entity::{node_mods, nodes};
use sea_orm::{ActiveValue::Set, DatabaseTransaction, EntityTrait};

use crate::domain::{graphs::Graph, nodes::{Node, error::NodeRepositoryError, repository::orm_to_domain::to_node}, posts::NewNode};

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
        
        result.insert(map_id, to_node(model));
    }

    Ok(result)
}

mod orm_to_domain {

    use entity::{ nodes::Model as NodeOrm };

    use crate::domain::nodes::{Node, Rarity};

    pub fn to_node(orm: NodeOrm)
        -> Node {
            Node { id: orm.id, context: orm.context, rarity: Rarity::from(orm.rarity), item_level: orm.item_level, note: orm.note, position_x: orm.position_x, position_y: orm.position_y, item_base_id: orm.item_base_id }
        }

}