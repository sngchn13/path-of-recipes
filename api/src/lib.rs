use crate::{app::{build_router, get_state, startup::host_and_port}, config::run_migration_from_env};

mod config;
mod app;
mod infrastructure;
mod domain;
mod shared;


#[tokio::main]
async fn start() -> anyhow::Result<()> {
    app::startup().ok();
    
    let state = get_state().await;
    
    if let Err(e) = run_migration_from_env(&state.conn).await {
        tracing::warn!("migration failed: {e}");
    }

    let router = build_router(state);

    let (host, port) = host_and_port();
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await.unwrap();
    
    axum::serve(listener, router).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}