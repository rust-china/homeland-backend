mod error;
mod logger;

use arel::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::setup()?;
    dotenvy::from_filename(".env")?;
    let _ = dotenvy::from_filename_override(".env.local");

    arel::db::visitor::init().await?;
    sqlx::migrate!().run(arel::db::visitor::get()?.pool()).await?;

    let count = entity::User::query().select_sql("count(*)").fetch_count().await?;
    log::info!("user count: {}", count);
    println!("Hello, world!");

    Ok(())
}
