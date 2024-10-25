mod config;
mod route;
mod handler;

use crate::config::Config;
use anyhow::{Context, Result};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;


#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from the `.env` file
    dotenv().ok();

    // Retrieve the value of the `DATABASE_URL` from .env file
    let config = Config::init();


    // Connect to `Sqlite` database
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect("")
        .await
        .context("Error: 🔥 unable to connect to database!")?;

    println!("✅ Successfully connected to database!");

    // Start the http server
    route::serve(config).await?;

    Ok(())
}