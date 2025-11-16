use std::{env, time::Duration};

use axum::{Router, routing::get};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::{debug, info};

mod config;


#[tokio::main]
async fn main() {
    config::init().ok();

    // let db = env::var("DATABASE_URL").expect("env load fail");
    // info!("{}", db);

    let app = Router::new()
        .route("/", get(|| async move {"hello world"}))
        .layer(TimeoutLayer::new(Duration::from_millis(3000)))
        .layer(TraceLayer::new_for_http());

    let host = env::var("APP_HOST").expect("APP_HOST not found");
    let port = env::var("APP_PORT").expect("APP_PORT not found");
    info!("ip={}, port= {}", &host, &port);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
