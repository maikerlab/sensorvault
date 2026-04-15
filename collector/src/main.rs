mod app_config;
mod ingestion;

use crate::ingestion::IngestionService;
use crate::ingestion::decoder::DecoderRegistry;
use crate::ingestion::decoder::mqtt::{RawMQTTDecoder, Zigbee2MQTTDecoder};
use anyhow::Result;
use app_config::AppConfig;
use sensorvault_infra::persistence::postgres::PostgresDatabase;

#[tokio::main]
async fn main() -> Result<()> {
    // Init logging
    tracing_subscriber::fmt::init();

    // Load config
    let config = AppConfig::load();

    // Connect to database
    let db =
        PostgresDatabase::connect(config.database.url, config.database.max_connections).await?;

    // Define used decoders + registry
    let decoder_registry = DecoderRegistry::new()
        .register(RawMQTTDecoder)
        .register(Zigbee2MQTTDecoder);

    // Create and run ingestion service
    let ingestion = IngestionService::new(db, decoder_registry);
    let mqtt_conf = config.mqtt;
    ingestion
        .run(
            mqtt_conf.host,
            mqtt_conf.port,
            mqtt_conf.username,
            mqtt_conf.password,
            mqtt_conf.subscribe_topics,
        )
        .await
}
