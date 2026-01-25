use std::collections::HashSet;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{node_mod::Placement, nodes::{ Context, Flags, Influence, Rarity}};

mod response;
pub use response::{PostResponse, PostSummary};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNode{
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
    pub mods: Vec<ModWithPlacement>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModWithPlacement{
    pub mod_id: i32,
    pub placement: Placement,
    pub fractured: bool,
}

#[derive(Debug, Serialize)]
pub struct CreatePostResponse{
    pub post_id: Uuid
}

impl CreatePostResponse {
    pub fn new(post_id: Uuid) -> Self {
        Self { post_id }
    }
}

impl IntoResponse for CreatePostResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}