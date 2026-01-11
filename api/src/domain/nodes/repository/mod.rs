use std::collections::{HashMap, HashSet};

use entity::{item_bases, item_types, node_mods, nodes, sea_orm_active_enums::Rarity};
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, FromQueryResult, QueryFilter, raw_sql};
use uuid::Uuid;

use crate::{domain::{graphs::Graph, item_bases::ItemBase, item_types::ItemType, nodes::{Node, error::NodeError}, posts::NewNode}, shared::to_node};

mod dto;
pub use dto::*;

pub async fn save_nodes_returning_map(new_nodes: Vec<NewNode>, graph: &Graph,txn: &DatabaseTransaction) -> Result<HashMap<String, Node>, NodeError>{
    
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

pub async fn find_nodes_by_graph_id(graph_id: &i32, conn: &DatabaseConnection) -> Result<Vec<Node>, NodeError>{
    
    let nodes: Vec<Node> = nodes::Entity::find().filter(nodes::Column::GraphId.eq(*graph_id)).all(conn).await?
        .into_iter()
        .map(|orm| to_node(orm))
        .collect();
    
    Ok(nodes)
}

pub async fn find_representative_nodes_by_mod_group_ids(mod_group_ids: &HashSet<i32>, conn: &DatabaseConnection) -> Result<Vec<Node>, NodeError>{

    let mod_group_size = mod_group_ids.len() as i64;

    let representative_nodes = nodes::Entity::find()
        .from_raw_sql(raw_sql!(
            Postgres,
            r#" 
            WITH node_stats AS (
                SELECT
                    n.id,
                    n.graph_id,
                    COUNT(DISTINCT mg.id) AS matched_group_count,
                    COUNT(m.id)            AS mod_count
                FROM mod_groups mg
                    JOIN mods      m  ON mg.id = m.mod_group_id
                    JOIN node_mods nm ON m.id = nm.mod_id
                    JOIN nodes     n  ON nm.node_id = n.id
                WHERE mg.id IN ({..mod_group_ids})
                GROUP BY
                    n.id,
                    n.graph_id
            ),
            candidates AS (
                SELECT *
                FROM node_stats
                WHERE matched_group_count = {mod_group_size}
            ),
            ranked AS (
                SELECT
                    c.*,
                    ROW_NUMBER() OVER (
                        PARTITION BY c.graph_id
                        ORDER BY c.mod_count DESC, c.id DESC
                    ) AS rn
                FROM candidates c
            )
            SELECT
                n.id,
                n.context,
                n.rarity::text AS rarity,
                n.item_level,
                n.note,
                n.position_x,
                n.position_y,
                n.item_base_id,
                n.graph_id
            FROM ranked r
            JOIN nodes n ON n.id = r.id
            WHERE r.rn = 1
        "#
        ))
        .all(conn)
        .await?;
    
    let representative_nodes: Vec<Node> = representative_nodes.into_iter().map(|n| to_node(n)).collect();

    Ok(representative_nodes)
}

pub async fn find_representative_nodes_by_posts(post_ids: Vec<Uuid>, conn: &DatabaseConnection) -> Result<HashMap<Uuid, Node>, NodeError>{

    if post_ids.is_empty() { return Ok(HashMap::new()) }

    let node_with_post_ids = NodeWithPostId::find_by_statement(raw_sql!(
        Postgres,
       r#"
       SELECT
            t.id,
            t.context,
            t.rarity::text AS rarity,
            t.item_level,
            t.note,
            t.position_x,
            t.position_y,
            t.item_base_id,
            t.graph_id,
            t.post_id
        FROM (
            SELECT
                p.id AS post_id,
                n.id,
                n.context,
                n.rarity,
                n.item_level,
                n.note,
                n.position_x,
                n.position_y,
                n.item_base_id,
                n.graph_id,
                COUNT(nm.mod_id) AS mod_count,
                ROW_NUMBER() OVER (
                    PARTITION BY p.id
                    ORDER BY COUNT(nm.mod_id) DESC, n.id DESC
                ) AS rn
            FROM posts p
                JOIN graphs g ON p.id = g.post_id
                JOIN nodes  n ON g.id = n.graph_id
                LEFT JOIN node_mods nm ON n.id = nm.node_id
            WHERE p.id IN ({..post_ids})
            GROUP BY
                p.id,
                n.id,
                n.context,
                n.rarity,
                n.item_level,
                n.note,
                n.position_x,
                n.position_y,
                n.item_base_id,
                n.graph_id
        ) AS t
        WHERE t.rn = 1
        "#
    )).all(conn).await?;
    
    let node_map_by_post_id: HashMap<Uuid, Node> = node_with_post_ids.into_iter().map(|np| {
        let (node, post_id) = np.divide();
        (post_id, node)
    }).collect();

    Ok(node_map_by_post_id)
}

pub async fn find_nodes_with_related_by_graph_id(graph_id: &i32, conn: &DatabaseConnection) 
-> Result<Vec<RawNodeAggregate>, NodeError>
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
            .ok_or(NodeError::ItemBaseNotFound(node.id.to_string()))?;
        let item_type = item_type
            .and_then(|orm| Some(ItemType::new(orm.id, orm.name)))
            .ok_or(NodeError::ItemTypeNotFound(item_base.id.to_string()))?;

        result.push(RawNodeAggregate::assembly(node, item_base, item_type));
    }

    Ok(result)
}

pub async fn find_nodes_with_related_by_node_id(node_ids: Vec<i32>, conn: &DatabaseConnection) 
-> Result<Vec<RawNodeAggregate>, NodeError>
{
    
    let rows = nodes::Entity::find()
        .filter(nodes::Column::Id.is_in(node_ids))
        .find_also(nodes::Entity, item_bases::Entity)
        .find_also(item_bases::Entity, item_types::Entity)
        .all(conn)
        .await?;

    let mut result = Vec::new();
    for (node, item_base, item_type) in rows {
        let node = to_node(node);
        let item_base = item_base
            .and_then(|orm| Some(ItemBase::new(orm.id, orm.name, orm.display_en, orm.display_kor, orm.item_type_id)))
            .ok_or(NodeError::ItemBaseNotFound(node.id.to_string()))?;
        let item_type = item_type
            .and_then(|orm| Some(ItemType::new(orm.id, orm.name)))
            .ok_or(NodeError::ItemTypeNotFound(item_base.id.to_string()))?;

        result.push(RawNodeAggregate::assembly(node, item_base, item_type));
    }

    Ok(result)
}

#[derive(Debug, FromQueryResult)]
struct NodeWithPostId {
    pub id: i32,
    pub context: Option<String>,
    pub rarity: Rarity,
    pub item_level: i32,
    pub note: Option<String>,
    pub position_x: f32,
    pub position_y: f32,
    pub item_base_id: i32,
    pub graph_id: i32,
    pub post_id: Uuid,
}

impl NodeWithPostId {
    pub fn divide(self) -> (Node, Uuid) {
        let node = Node {
            id: self.id,
            context: self.context,
            rarity: super::Rarity::from(self.rarity),
            item_level: self.item_level,
            note: self.note,
            position_x: self.position_x,
            position_y: self.position_y,
            item_base_id: self.item_base_id,
        };

        (node, self.post_id)
    }
}