use std::collections::{HashMap};

use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::info;
use uuid::Uuid;

use crate::domain::{causes::find_all_returning_map, edges::{find_edge_aggregate_by_graph_id, find_edges_by_graph_id, save_edges_with_causes}, graphs::{GraphAggregate, find_graph_by_id, save_graph}, mods::{ModAggregate, UnplacedModAggregate, find_all_mods_with_related_by_mod_ids}, node_mod::{NodeMod, Placement, find_node_mods_from_node_ids}, nodes::{NodeAggregate, RawNodeAggregate, find_nodes_with_related_by_graph_id, save_nodes_returning_map}, posts::{Post, PostServiceError, presentation::NewPost, repository::{find_post_by_id, save_post}, service::{PostAggregate, PostService}}};

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

    async fn find_post_with_related(&self, post_id: Uuid) -> Result<PostAggregate, PostServiceError>{
        let conn = &self.database_connection;
        //포스트 가져오기
        let post = find_post_by_id(post_id, conn).await?.ok_or(PostServiceError::NotFoundPost(post_id.to_string()))?;

        //그래프 가져오기
        let graph = find_graph_by_id(&post.id, conn).await?.ok_or(PostServiceError::NotFoundGraph(post_id.to_string()))?;

        //노드 가져오기
        let raw_nodes = find_nodes_with_related_by_graph_id(&graph.id, conn).await?;
        //노드 모드 가져오기
        let node_mod_map = find_node_mods_from_node_ids(raw_nodes.iter().map(|n| n.id).collect(), conn).await?;
        info!("node map = {:#?}", node_mod_map);
        //모드 가져오기
        let unplaced_mod_map = find_all_mods_with_related_by_mod_ids(node_mod_map.values().flat_map(|mods| mods.iter()).map(|m| m.mod_id).collect(), conn).await?;
        //노드 조합하기
        let nodes = assemble_node(raw_nodes, node_mod_map, unplaced_mod_map)?;

        //엣지 가져오기
        let edges = find_edge_aggregate_by_graph_id(&graph.id, conn).await?;

        //그래프 조합하기
        let graph = GraphAggregate::assemble_graph(graph, nodes.into_iter().collect(), edges.into_iter().collect());

        let post = PostAggregate::assemble_post(post, graph);

        Ok(post)
    }
}

fn assemble_node(raw_nodes: Vec<RawNodeAggregate>, mut node_mod_lookup: HashMap<i32, Vec<NodeMod>>, mod_lookup: HashMap<i32, UnplacedModAggregate>) -> Result<Vec<NodeAggregate>, PostServiceError> {
    let mut result = Vec::new();

    for raw_node in raw_nodes {
        let mut mod_aggregates = Vec::new();
        for node_mod in node_mod_lookup.remove(&raw_node.id).unwrap_or_default() {
            let mod_id = node_mod.mod_id;
            let placement = node_mod.placement;

            let mod_aggregate = mod_lookup.get(&mod_id).ok_or(PostServiceError::AssembleErr(format!("mod not found with mod_id{}", mod_id)))?.clone().place_mode(placement);
            mod_aggregates.push(mod_aggregate);
        }

        let node = raw_node.add_mods(mod_aggregates);
        result.push(node);
    }

    Ok(result)
}