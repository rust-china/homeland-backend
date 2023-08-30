use arel::prelude::*;
mod settings;
mod logger;

use settings::SETTINGS;
use log::{info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(".env")?;
    arel::visitor::init().await?;
    logger::setup()?;
    info!("start server app: {}", SETTINGS.server.name);
    let port = SETTINGS.server.port;
    info!("user port: {}", port);
    let count = entity::User::query().select_sql("count(*)").fetch_count().await?;
    info!("user count: {}", count);
    Ok(())
}
