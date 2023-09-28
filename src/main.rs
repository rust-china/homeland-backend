mod app;
mod error;
mod logger;
mod settings;

use app::{AppState, APP_STATE};
use arel::prelude::*;
use error::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_env();
    logger::setup()?;
    arel::db::visitor::init().await?;
    sqlx::migrate!().run(arel::db::visitor::get()?.pool()).await?;
    app::init_app_state().await?;

    let count = entity::User::query().select_sql("count(*)").fetch_count().await?;
    log::info!("user count: {}", count);
    println!("Hello, world!");

    let port = settings::SETTINGS.get_string("database.name")?;
    log::info!("user port: {}", port);

    let addr = format!("0.0.0.0:{}", std::env::var("SERVE_PORT")?).parse()?;
    app::listen(addr).await
}

fn load_env() {
    dotenvy::from_filename(".env").unwrap();
    let _ = dotenvy::from_filename_override(".env.local");
}
