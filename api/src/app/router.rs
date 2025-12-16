use std::time::Duration;

use axum::{Router, http::HeaderValue, routing::get};
use tower_http::{cors::{Any, CorsLayer}, timeout::TimeoutLayer, trace::TraceLayer};

use crate::{app::state::AppState, domain::mod_groups::get_mod_groups_by_item_type};

pub fn build_router(state: AppState) -> Router{

    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:5173") //todo cors설정 제대로 하기
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api", get(|| async move {"hello world"}))
        .route("/api/mod-groups", get( get_mod_groups_by_item_type))
        .layer(TimeoutLayer::new(Duration::from_millis(3000)))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}