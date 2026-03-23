mod config;
mod ingestion;

use crate::ingestion::IngestionService;
use crate::ingestion::decoder::raw::RawMQTTDecoder;
use crate::ingestion::decoder::{DecoderRegistry, SensorDataDecoder};
use anyhow::Result;
use config::AppConfig;
use infra::persistence::postgres::PostgresDatabase;

#[tokio::main]
async fn main() -> Result<()> {
    // Init logging
    tracing_subscriber::fmt::init();

    // Load config
    let config = AppConfig::load();

    // Connect to database
    let db = PostgresDatabase::connect(config.database.url, config.database.max_connections).await?;

    // Define used decoders + registry
    let decoders: Vec<Box<dyn SensorDataDecoder>> = vec![Box::new(RawMQTTDecoder)];
    let decoder_registry = DecoderRegistry::new(decoders);

    // Create and run ingestion service
    let ingestion = IngestionService::new(db, decoder_registry);
    ingestion.run(config.mqtt.host, config.mqtt.port).await
}
