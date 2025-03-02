use config::Config;
use dotenv::dotenv;
use std::error::Error;

use infrastructure::app_setup::{initialize_dependencies, spawn_background_tasks};

mod config;
mod infrastructure;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let config = Config::from_env()?;

    let deps = initialize_dependencies(&config).await?;
    spawn_background_tasks(deps.event_consumer, deps.notification_service).await;

    Ok(())
}
