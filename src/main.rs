use crate::app::{build_router, get_state, startup::host_and_port};

mod config;
mod app;
mod infrastructure;


#[tokio::main]
async fn main() {
    app::startup().ok();
    
    let state = get_state().await;
    let router = build_router(state);

    let (host, port) = host_and_port();
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await.unwrap();
    
    axum::serve(listener, router).await.unwrap();
}
