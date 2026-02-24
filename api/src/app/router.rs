use std::time::Duration;

use axum::{Router, http::{HeaderValue, Method, header}, middleware, routing::{get, post}};
use tower_http::{cors::{CorsLayer}, timeout::TimeoutLayer, trace::TraceLayer};

use crate::{app::{layer::anon_id_middleware, state::AppState}, domain::{comments::{get_comments_by_conditon, get_replies_by_condition, post_new_comment}, images::get_image_by_id, item_bases::get_item_bases_by_item_name, mod_groups::get_mod_groups_by_item_type, posts::{get_post_by_id, get_posts_by_condition, post_new_post}, reactions::{post_new_comment_reaction, post_new_post_reaction}}};

pub fn build_router(state: AppState) -> Router{

    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:5173") //todo cors설정 제대로 하기
        ])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::DELETE, Method::PUT])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT, header::ORIGIN]);

    Router::new()
        .route("/api", get(|| async move {"hello world"}))
        .route("/api/mod-groups", get( get_mod_groups_by_item_type))
        .route("/api/item-bases", get(get_item_bases_by_item_name))
        .route("/api/images/{id}", get(get_image_by_id))
        .route("/api/posts", post(post_new_post))
        .route("/api/posts", get(get_posts_by_condition))
        .route("/api/posts/{post_id}", get(get_post_by_id))
        .route("/api/posts/{post_id}/comments", post(post_new_comment))
        .route("/api/posts/{post_id}/comments", get(get_comments_by_conditon))
        .route("/api/posts/{post_id}/reactions", post(post_new_post_reaction))
        .route("/api/comments/{comment_id}/reactions", post(post_new_comment_reaction))
        .route("/api/comments/{comment_id}/comments", get(get_replies_by_condition))
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(state, anon_id_middleware))
        .layer(TimeoutLayer::new(Duration::from_millis(3000)))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}