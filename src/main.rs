use arel::prelude::*;
mod logger;
mod settings;

use settings::SETTINGS;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(".env")?;
    let _ = dotenvy::from_filename_override(".env.local");

    arel::visitor::init().await?;
    logger::setup()?;
    log::info!("start server app: {}", SETTINGS.server.name);
    let port = SETTINGS.server.port;
    log::info!("user port: {}", port);
    let count = entity::User::query().select_sql("count(*)").fetch_count().await?;
    log::info!("user count: {}", count);
    Ok(())
}
