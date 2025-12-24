use serde::Deserialize;

use crate::domain::posts::Rarity;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNode{
    pub id: String,
    pub context: Option<String>,
    pub rarity: Rarity,
    pub item_level: i32,
    pub note: Option<String>,
    pub position_x: f32,
    pub position_y: f32,
    pub item_base_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewEdge{
    pub id: String,
    pub source_id: Option<String>,
    pub target_id: Option<String>,
    pub causes: Vec<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGraph{
    pub nodes: Vec<NewNode>,
    pub edges: Vec<NewEdge>,
    pub description: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPost{
    pub title: String,
    pub password: String,
    pub graph: NewGraph,
}