use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::info;
use uuid::Uuid;

use crate::domain::{causes::find_all_returning_map, edges::save_edges_with_causes, graphs::save_graph, nodes::{ save_nodes_returning_map}, posts::{PostServiceError, presentation::NewPost, repository::save_post, service::PostService}};

pub struct PostServiceImpl{
    database_connection: DatabaseConnection,
}

impl PostServiceImpl {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self { database_connection }
    }
}

#[async_trait::async_trait]
impl PostService for PostServiceImpl {
    async fn save_posts_with_related(&self, new_post: NewPost) -> Result<Uuid, PostServiceError>{
        //트랜잭션 시작
        let txn = self.database_connection.begin().await?;
        
        //todo new_post 검증하기
        
        //post 저장
        let post = save_post(&txn, &new_post).await?;

        let txt = format!("{:#?}", post);
        info!("{}", txt);
        
        //graph 저장
        let graph = save_graph(&txn, &new_post.graph, &post).await?;

        //노드 및 노드모드 저장 map형태로 <string, Node>
        let node_map = save_nodes_returning_map(new_post.graph.nodes, &graph, &txn).await?;
        
        //엣지 및 엣지커즈 저장
        let cause_lookup = find_all_returning_map(&txn).await?;
        save_edges_with_causes(new_post.graph.edges, node_map, graph.id, cause_lookup, &txn).await?;

        //트랜잭션 종료
        txn.commit().await?;

        Ok(post.id)
    }
}