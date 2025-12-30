use entity::{graphs, sea_orm_active_enums::Verification};
use sea_orm::{ActiveValue::Set, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;

use crate::{domain::{graphs::{Graph, error::GraphRepositoryError}, posts::{NewGraph, Post}}, shared::to_graph};

mod dto;
pub use dto::*;


pub async fn save_graph(txn: &DatabaseTransaction, new_graph: &NewGraph, post: &Post) -> Result<Graph, GraphRepositoryError>{

    let model = graphs::ActiveModel {
        description: Set(new_graph.description.clone()),
        verification: Set(Verification::Valid), //todo 검증 로직 추가하면 pending으로 바꾸기
        post_id: Set(post.id),
        ..Default::default()
    };

    let saved = graphs::Entity::insert(model).exec_with_returning(txn).await?;

    Ok(to_graph(saved))
}

pub async fn find_graph_by_id(post_id: &Uuid, conn: &DatabaseConnection) -> Result<Option<Graph>, GraphRepositoryError>{
    let graph = graphs::Entity::find().filter(graphs::Column::PostId.eq(*post_id)).one(conn).await?.and_then(|v| Some(to_graph(v)));

    Ok(graph)
}

