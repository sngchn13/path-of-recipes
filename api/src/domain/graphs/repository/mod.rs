use entity::{graphs, sea_orm_active_enums::Verification};
use sea_orm::{ActiveValue::Set, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;

use crate::{domain::{graphs::{Graph, error::GraphError}, posts::{NewGraph, Post}}, shared::to_graph};

mod dto;
pub use dto::*;


pub async fn save_graph(txn: &DatabaseTransaction, new_graph: &NewGraph, post: &Post) -> Result<Graph, GraphError>{

    let model = graphs::ActiveModel {
        description: Set(new_graph.description.clone()),
        verification: Set(Verification::Valid), //todo 검증 로직 추가하면 pending으로 바꾸기
        post_id: Set(post.id),
        ..Default::default()
    };

    let saved = graphs::Entity::insert(model).exec_with_returning(txn).await?;

    Ok(to_graph(saved))
}

pub async fn find_graph_by_post_id(post_id: &Uuid, conn: &DatabaseConnection) -> Result<Option<Graph>, GraphError>{
    let graph = graphs::Entity::find().filter(graphs::Column::PostId.eq(*post_id)).one(conn).await?.and_then(|v| Some(to_graph(v)));

    Ok(graph)
}

pub async fn delete_graph_by_post_id(post_id: Uuid, txn: &DatabaseTransaction) -> Result<(), GraphError>{
    let res = graphs::Entity::delete_many()
        .filter(graphs::Column::PostId.eq(post_id))
        .exec(txn)
        .await?;

    if res.rows_affected > 1 {
        return Err(GraphError::UnexpectedAffectedRows(res.rows_affected, post_id));
    }

    Ok(())
}
