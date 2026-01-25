use std::collections::HashSet;

use serde::{Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::{edges::EdgeAggregate, graphs::GraphAggregate, mods::ModAggregate, node_mod::Placement, nodes::{Context, Flags, Influence, NodeAggregate, Rarity}, posts::{Post, service::PostAggregate}};


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostResponse {
    id: Uuid,
    title: String,
    content: String,
    view: i32,
    like: i32,
    dislike: i32,
    graph: GraphResponse,

    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339::option")]
    updated_at: Option<OffsetDateTime>,
}

impl From<PostAggregate> for PostResponse {
    fn from(value: PostAggregate) -> Self {
        Self {
            id: value.id,
            title: value.title,
            content: value.content,
            view: value.view,
            like: value.like,
            dislike: value.dislike,
            graph: value.graph.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GraphResponse {
    pub id: i32,
    pub description: Option<String>,
    pub nodes: Vec<NodeResponse>,
    pub edges: Vec<EdgeResponse>
}

impl From<GraphAggregate> for GraphResponse {
    fn from(value: GraphAggregate) -> Self {
        Self {
            id: value.id,
            description: value.description,
            nodes: value
                .nodes
                .into_iter()
                .map(NodeResponse::from)
                .collect(),
            edges: value
                .edges
                .into_iter()
                .map(EdgeResponse::from)
                .collect(),
        }
    }
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NodeResponse {
    pub id: String,
    pub context: Context,
    pub rarity: Rarity,
    pub item_level: i32,
    pub quality: i32,
    pub flags: Flags,
    pub influences: HashSet<Influence>,
    pub note: Option<String>,
    pub position_x: f32,
    pub position_y: f32,
    pub item_base_id: i32,
    pub item_base_name: String,
    pub item_base_display_en: Option<String>,
    pub item_base_display_kor: Option<String>,
    pub item_type_id: i32,
    pub item_type_name: String,
    pub mods: Vec<ModResponse>
}

impl From<NodeAggregate> for NodeResponse {
    fn from(value: NodeAggregate) -> Self {
        Self {
            id: format!("node-{}", value.id),
            context: value.context,
            rarity: value.rarity,
            item_level: value.item_level,
            quality: value.quality,
            flags: value.flags,
            influences: value.influences,
            note: value.note,
            position_x: value.position_x,
            position_y: value.position_y,
            item_base_id: value.item_base_id,
            item_base_name: value.item_base_name,
            item_base_display_en: value.item_base_display_en,
            item_base_display_kor: value.item_base_display_kor,
            item_type_id: value.item_type_id,
            item_type_name: value.item_type_name,
            mods: value
                .mods
                .into_iter()
                .map(ModResponse::from)
                .collect(),
        }
    }
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ModResponse{
    pub id: i32,
    pub name_en: String,
    pub name_kor: String,
    pub display_en: String,
    pub display_kor: String,
    pub req_item_level: i32,
    pub weight: Option<i16>,
    pub tier: Option<i16>,
    pub deprecated_version: Option<String>,
    pub mod_group_id: i32,
    pub mod_group_value: String,
    pub mod_generation_type_id: i32,
    pub tags: HashSet<String>,
    pub families: HashSet<String>,
    pub section: String,
    pub placement: Placement,
    pub fractured: bool,
}

impl From<ModAggregate> for ModResponse  {
    fn from(value: ModAggregate) -> Self {
        Self {
            id: value.id,
            name_en: value.name_en,
            name_kor: value.name_kor,
            display_en: value.display_en,
            display_kor: value.display_kor,
            req_item_level: value.req_item_level,
            weight: value.weight,
            tier: value.tier,
            deprecated_version: value.deprecated_version,
            mod_group_id: value.mod_group_id,
            mod_group_value: value.mod_group_value,
            mod_generation_type_id: value.mod_generation_type.id,
            tags: value
                .tags
                .into_iter()
                .map(|t| t.name)
                .collect(),
            families: value
                .families
                .into_iter()
                .map(|f| f.name)
                .collect(),
            section: value.section.name,
            placement: value.placement,
            fractured: value.fractured,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EdgeResponse {
    pub id: String,
    pub note: Option<String>,
    pub source_id: Option<String>,
    pub target_id: Option<String>,
    pub causes: Vec<String>
}

impl From<EdgeAggregate> for EdgeResponse {
    fn from(value: EdgeAggregate) -> Self {
        Self {
            id: format!("edge-{}", value.id),
            note: value.note,
            source_id: value.source_id.and_then(|id| Some(format!("node-{}", id))),
            target_id: value.target_id.and_then(|id| Some(format!("node-{}", id))),
            causes: value
                .causes
                .into_iter()
                .map(|c| c.to_string())
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostSummary {
    pub id: Uuid,
    pub title: String,
    pub view: i32,
    pub like: i32,
    pub dislike: i32,
    
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
    pub representative_node: NodeResponse,
}

impl PostSummary {
    pub fn from(post: Post, node: NodeAggregate) -> Self{
        Self { 
            id: post.id, 
            title: post.title, 
            view: post.view, 
            like: post.like, 
            dislike: post.dislike, 
            created_at: post.created_at, 
            updated_at: post.updated_at, 
            representative_node: NodeResponse::from(node) 
        }
    }
}