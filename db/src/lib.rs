use common::settings::Settings;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use anyhow::Result;

pub mod registry;
pub mod data;

pub async fn connect_postgres() -> Result<PgPool> {
    let settings = Settings::load();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(settings.database_url.as_str())
        .await?;

    Ok(pool)
}