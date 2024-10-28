mod config;
mod route;
mod handler;

use crate::config::Config;
use anyhow::Result;
use dotenv::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from the `.env` file
    tracing_subscriber::fmt::init();
    dotenv().ok();

    // Retrieve the value of the `DATABASE_URL` from .env file
    let config = Config::init();

    info!("✅ Successfully connected to database!");

    // Start the http server
    route::serve(config).await?;

    Ok(())
}