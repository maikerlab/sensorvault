use anyhow::Result;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

pub mod data;
pub mod registry;

pub async fn connect_postgres(url: String) -> Result<PgPool> {
    info!("connecting to postgres: {}", url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await?;

    Ok(pool)
}
