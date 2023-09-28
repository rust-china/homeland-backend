use config::{Config, Environment, File};
use once_cell::sync::Lazy;

use crate::load_env;
// use std::sync::RwLock;

pub static SETTINGS: Lazy<Config> = Lazy::new(|| {
    load_env();
    let run_mode = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".into());
    let mut builder = Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name(&format!("config/{run_mode}")).required(false))
        .add_source(File::with_name("config/local").required(false))
        .add_source(Environment::default().separator("__"));

    if let Ok(port) = std::env::var("PORT") {
        builder = builder.set_override("server.port", port).unwrap();
    }
    if let Ok(url) = std::env::var("DATABASE_URL") {
        builder = builder.set_override("database.url", url).unwrap();
    }

    builder.build().unwrap()
});
