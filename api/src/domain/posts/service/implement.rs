use std::collections::{HashMap, HashSet};

use sea_orm::{DatabaseConnection, TransactionTrait};
use tracing::info;
use uuid::Uuid;

use crate::{domain::{causes::find_all_returning_map, edges::{find_edge_aggregate_by_graph_id, save_edges_with_causes}, graphs::{GraphAggregate, find_graph_by_post_id, save_graph}, mods::{ModAggregate, UnplacedModAggregate, find_all_mods_with_related_by_mod_ids}, node_mod::{NodeMod, Placement, find_node_mods_from_node_ids}, nodes::{NodeAggregate, RawNodeAggregate, find_nodes_with_related_by_graph_id, find_nodes_with_related_by_node_id, find_representative_nodes_by_mod_group_ids, find_representative_nodes_by_posts, save_nodes_returning_map}, posts::{Post, PostError, presentation::{NewPost, PostOrder, PostSearchCondition}, repository::{find_all_posts, find_post_by_id, find_posts_by_node_id, save_post}, service::{PostAggregate, PostService}}}, shared::{Page, Pagination, SortDirection}};

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
    async fn save_posts_with_related(&self, new_post: NewPost) -> Result<Uuid, PostError>{
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

    async fn find_post_with_related(&self, post_id: Uuid) -> Result<PostAggregate, PostError>{
        let conn = &self.database_connection;
        //포스트 가져오기
        let post = find_post_by_id(post_id, conn).await?.ok_or(PostError::NotFoundPost(post_id.to_string()))?;

        //그래프 가져오기
        let graph = find_graph_by_post_id(&post.id, conn).await?.ok_or(PostError::GraphMissingForPost(post.id.to_string()))?;

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

    async fn find_posts_with_representative_node(&self, mod_group_ids: HashSet<i32>, pagination: Pagination, order_by: PostOrder,direction: SortDirection) 
    -> Result<Page<(Post, NodeAggregate)>, PostError>
    {
        //요구하는 모드의 조건이 없는 경우
        if(mod_group_ids.is_empty()){
            //포스트 시작 + 페이지네이팅
            let paged_posts = find_all_posts(pagination, order_by, direction, &self.database_connection).await?;
            //대표노드 찾기
            let mut post_node_relation = find_representative_nodes_by_posts(paged_posts.content.iter().map(|p| p.id).collect(), &self.database_connection).await?;
            
            //노드 만들기
            let raw_nodes = find_nodes_with_related_by_node_id(post_node_relation.values().map(|n| n.id).collect(), &self.database_connection).await?;
            let node_mod_map = find_node_mods_from_node_ids(raw_nodes.iter().map(|n|n.id).collect(), &self.database_connection).await?;
            let mod_map = find_all_mods_with_related_by_mod_ids(node_mod_map.values().flat_map(|mods| mods.iter()).map(|m| m.mod_id).collect(), &self.database_connection).await?;
            let mut node_map: HashMap<i32, NodeAggregate> = assemble_node(raw_nodes, node_mod_map, mod_map)?.into_iter().map(|n| (n.id, n)).collect();

            //조립 시작
            let pageable = paged_posts.pageable;
            let content = paged_posts.content;
            let mut result_content = Vec::new();


            info!("posts len = {}, post_node_relation len = {}",
                content.len(),
                post_node_relation.len()
            );

            for post in content {
                let node_id = post_node_relation.remove(&post.id).ok_or(PostError::AssembleErr(format!("node_id not found for post_id: {}", post.id)))?.id;
                let node = node_map.remove(&node_id).ok_or(PostError::AssembleErr(format!("node not found for node_id: {}", node_id)))?;
                
                result_content.push((post, node));
            }

            return Ok(Page::new(result_content, pageable))
        }

        //모드 조건이 있는 경우: 대표노드 찾고 이후에 포스트 찾기+페이지네이팅
        let representative_nodes = find_representative_nodes_by_mod_group_ids(&mod_group_ids, &self.database_connection).await?;
        let raw_nodes = find_nodes_with_related_by_node_id(representative_nodes.iter().map(|n| n.id).collect(), &self.database_connection).await?;
        let node_mod_map = find_node_mods_from_node_ids(raw_nodes.iter().map(|rn| rn.id).collect(), &self.database_connection).await?;
        let mod_map = find_all_mods_with_related_by_mod_ids(node_mod_map.values().flat_map(|mods| mods.iter()).map(|m| m.mod_id).collect(), &self.database_connection).await?;

        let representative_nodes = assemble_node(raw_nodes, node_mod_map, mod_map)?;
        let representative_node_map: HashMap<i32, NodeAggregate> = representative_nodes.into_iter().map(|n| (n.id, n)).collect();

        //노드에 맞는 포스트 찾기
        let paged_post = find_posts_by_node_id(representative_node_map.keys().map(|k|k.clone()).collect(), pagination, order_by, direction, &self.database_connection).await?;

        //페이지랑 포스트 연결하고 응답하기 Page(Post, NodeAggregate)
        let result_content = paged_post.content.into_iter()
            .map(|(post, node_id)| {
                let node = representative_node_map.get(&node_id)
                    .cloned()
                    .ok_or(PostError::AssembleErr(format!("node not found with id:{}", node_id)))?;
                Ok((post, node))
            })
            .collect::<Result<Vec<(Post, NodeAggregate)>, PostError>>()?;
        
        Ok(Page::new(result_content, paged_post.pageable))
    }
}

fn assemble_node(raw_nodes: Vec<RawNodeAggregate>, mut node_mod_lookup: HashMap<i32, Vec<NodeMod>>, mod_lookup: HashMap<i32, UnplacedModAggregate>) -> Result<Vec<NodeAggregate>, PostError> {
    let mut result = Vec::new();

    for raw_node in raw_nodes {
        let mut mod_aggregates = Vec::new();
        for node_mod in node_mod_lookup.remove(&raw_node.id).unwrap_or_default() {
            let mod_id = node_mod.mod_id;

            let mod_aggregate = mod_lookup.get(&mod_id).ok_or(PostError::AssembleErr(format!("mod not found with mod_id{}", mod_id)))?.clone().place_mode(&node_mod);
            mod_aggregates.push(mod_aggregate);
        }

        let node = raw_node.add_mods(mod_aggregates);
        result.push(node);
    }

    Ok(result)
}