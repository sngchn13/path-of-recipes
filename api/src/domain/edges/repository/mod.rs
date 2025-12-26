use std::collections::HashMap;
use entity::{edge_causes, edges};
use sea_orm::{ActiveValue::Set, DatabaseTransaction, EntityTrait};

use crate::domain::{causes::Cause, edges::{error::EdgeRepositoryError}, nodes::Node, posts::NewEdge};

pub async fn save_edges_with_causes(new_edges: Vec<NewEdge>, node_map: HashMap<String, Node>, graph_id: i32, cause_lookup: HashMap<String, Cause>, txn: &DatabaseTransaction)
    -> Result<(), EdgeRepositoryError>
{
    for new_edge in new_edges {

        //엣지 저장
        let optional_source_id = new_edge
            .source_id
            .as_ref()                     
            .and_then(|id| node_map.get(id)) 
            .map(|node| node.id.clone());  

        let optional_target_id = new_edge
            .target_id
            .as_ref()                     
            .and_then(|id| node_map.get(id)) 
            .map(|node| node.id.clone());

        let model = edges::ActiveModel{
            note: Set(new_edge.note),
            source_id: Set(optional_source_id),
            target_id: Set(optional_target_id),
            graph_id: Set(graph_id),
            ..Default::default()
        };
        let model = edges::Entity::insert(model).exec_with_returning(txn).await?;

        //커즈 저장
        let causes = new_edge.causes;
        let mut cause_models = Vec::new();
        for (index, cause) in causes.iter().enumerate() {
            let cause_id = cause_lookup.get(cause).ok_or(EdgeRepositoryError::CauseNameNotFound(cause.clone()))?.id;
            
            let cause_model = edge_causes::ActiveModel{
                edge_id: Set(model.id),
                cause_id: Set(cause_id),
                sequence: Set((index * 10) as i32),
            };
            cause_models.push(cause_model);
        }
        edge_causes::Entity::insert_many(cause_models).exec(txn).await?;
        
    }
    
    Ok(())
}

mod orm_to_domain {

    use entity::{ edges::Model as EdgeOrm };

    use crate::domain::edges::entity::Edge;

    pub fn _to_edge(orm: EdgeOrm)
        -> Edge {
            Edge::new(
                orm.id,
                orm.note,
                orm.source_id,
                orm.target_id,
                orm.graph_id,
            )
        }

}