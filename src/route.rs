use crate::handler::{discord_seafile_redirect, discord_seafile_redirect_head, discord_seafile_transformer, handler_404, home_handler};
use std::sync::Arc;

use anyhow::Result;
use axum::{
    routing::get,
    Router,
};
use axum::routing::head;
use tokio::sync::RwLock;
use tracing::info;
use crate::config::Config;

pub async fn serve(state: Config) -> Result<()> {
    info!("initializing router…");

    // Create the router using the application state
    let app = create_router(state);

    let port = 8082_u16;

    // Bind the server to the specified address and port
    let address = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await?;

    info!("🚀 router initialized, now listening on port {}", port);

    // Start serving incoming connections
    axum::serve(address, app.into_make_service()).await?;

    Ok(())
}

fn create_router(state: Config) -> Router {
    // General router of our application
    Router::new()
        .route("/", get(home_handler))
        .route("/:id", get(discord_seafile_transformer))

        // ඞ
        .route("/:id/file.:ext", head(discord_seafile_redirect_head))
        .route("/:id/file.:ext", get(discord_seafile_redirect))
        .with_state(Arc::new(RwLock::new(state)))
        .fallback(handler_404) // Add a Fallback service for handling unknown paths
}