use std::time::Duration;

use axum::{Router, routing::get};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::app::state::AppState;

pub fn build_router(state: AppState) -> Router{
    Router::new()
        .route("/", get(|| async move {"hello world"}))
        .with_state(state)
        .layer(TimeoutLayer::new(Duration::from_millis(3000)))
        .layer(TraceLayer::new_for_http())
}