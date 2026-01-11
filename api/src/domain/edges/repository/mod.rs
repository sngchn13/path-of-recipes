use std::collections::HashMap;
use entity::{causes, edge_causes, edges};
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, QueryOrder};

use crate::{domain::{causes::Cause, edges::{entity::Edge, error::EdgeError}, nodes::Node, posts::NewEdge}, shared::{to_cause, to_edge}};

mod dto;
pub use dto::*;

pub async fn save_edges_with_causes(new_edges: Vec<NewEdge>, node_map: HashMap<String, Node>, graph_id: i32, cause_lookup: HashMap<String, Cause>, txn: &DatabaseTransaction)
    -> Result<(), EdgeError>
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
            let cause_id = cause_lookup.get(cause).ok_or(EdgeError::CauseNameNotFound(cause.clone()))?.id;
            
            let cause_model = edge_causes::ActiveModel{
                edge_id: Set(model.id),
                cause_id: Set(cause_id),
                sequence: Set(((index + 1) * 10) as i32),
            };
            cause_models.push(cause_model);
        }
        edge_causes::Entity::insert_many(cause_models).exec(txn).await?;
        
    }
    
    Ok(())
}

pub async fn find_edges_by_graph_id(graph_id: &i32, conn: &DatabaseConnection) -> Result<Vec<Edge>, EdgeError>{

    let edges: Vec<Edge> = edges::Entity::find().filter(edges::Column::GraphId.eq(*graph_id)).all(conn).await?
        .into_iter()
        .map(|orm| to_edge(orm))
        .collect();

    Ok(edges)
}

pub async fn find_edge_aggregate_by_graph_id(graph_id: &i32, conn: &DatabaseConnection) -> Result<Vec<EdgeAggregate>, EdgeError>{

    let edge_with_causes = edges::Entity::find()
        .filter(edges::Column::GraphId.eq(*graph_id))
        .order_by_asc(edge_causes::Column::Sequence)//시퀸스 순으로 정렬
        .find_with_related(causes::Entity)
        .all(conn)
        .await?;

    let edges: Vec<EdgeAggregate> = edge_with_causes.into_iter()
        .map(|(edge, causes)| EdgeAggregate::attach_causes(to_edge(edge), causes.into_iter().map(|c|to_cause(c)).collect()))
        .collect();

    Ok(edges)
}