use std::time::Duration;

use axum::{Router, routing::get};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::{app::state::AppState, domain::mod_groups::get_mod_groups_by_item_type};

pub fn build_router(state: AppState) -> Router{
    Router::new()
        .route("/", get(|| async move {"hello world"}))
        .route("/mod-groups", get( get_mod_groups_by_item_type))
        .layer(TimeoutLayer::new(Duration::from_millis(3000)))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}