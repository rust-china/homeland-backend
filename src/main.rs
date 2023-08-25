use arel::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(".env")?;
    arel::visitor::init().await?;
    println!("Hello, world!");

    let count = entity::User::query().select_sql("count(*)").fetch_count().await?;
    println!("user count: {}", count);

    Ok(())
}
