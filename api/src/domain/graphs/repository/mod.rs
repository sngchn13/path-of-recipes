use entity::{graphs, sea_orm_active_enums::Verification};
use sea_orm::{ActiveValue::Set, DatabaseTransaction, EntityTrait};

use crate::domain::{graphs::{Graph, error::GraphRepositoryError, repository::orm_to_domain::to_post}, posts::{NewGraph, Post}};



pub async fn save_graph(txn: &DatabaseTransaction, new_graph: &NewGraph, post: &Post) -> Result<Graph, GraphRepositoryError>{

    let model = graphs::ActiveModel {
        description: Set(new_graph.description.clone()),
        verification: Set(Verification::Valid), //todo 검증 로직 추가하면 pending으로 바꾸기
        post_id: Set(post.id),
        ..Default::default()
    };

    let saved = graphs::Entity::insert(model).exec_with_returning(txn).await?;

    Ok(to_post(saved))
}

mod orm_to_domain {

    use entity::{ graphs::Model as GraphOrm };

    use crate::domain::graphs::{Graph, Verification};

    pub fn to_post(orm: GraphOrm)
        -> Graph {
            Graph::new(
                orm.id,
                orm.description,
                Verification::from(orm.verification),
                orm.post_id,
            )
        }

}