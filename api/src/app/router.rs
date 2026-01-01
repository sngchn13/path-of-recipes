use std::time::Duration;

use axum::{Router, http::HeaderValue, routing::{get, post}};
use tower_http::{cors::{Any, CorsLayer}, timeout::TimeoutLayer, trace::TraceLayer};

use crate::{app::state::AppState, domain::{images::get_image_by_id, item_bases::get_item_bases_by_item_name, mod_groups::get_mod_groups_by_item_type, posts::{get_post_by_id, get_posts_by_condition, post_new_post}}};

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
        .route("/api/item-bases", get(get_item_bases_by_item_name))
        .route("/api/images/{id}", get(get_image_by_id))
        .route("/api/posts", post(post_new_post))
        .route("/api/posts", get(get_posts_by_condition))
        .route("/api/posts/{post_id}", get(get_post_by_id))
        .layer(TimeoutLayer::new(Duration::from_millis(3000)))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}