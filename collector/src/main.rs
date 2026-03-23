mod ingestion;
mod persistence;

use crate::ingestion::IngestionService;
use crate::ingestion::decoder::raw::RawMQTTDecoder;
use crate::ingestion::decoder::{DecoderRegistry, SensorDataDecoder};
use crate::persistence::Database;
use anyhow::Result;
use core::settings::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // init logging
    tracing_subscriber::fmt::init();

    // Load config
    let settings = AppConfig::load();

    // Connect persistence
    let db = Database::connect(settings.database).await?;

    // Define decoders
    let decoders: Vec<Box<dyn SensorDataDecoder>> = vec![Box::new(RawMQTTDecoder)];
    let decoder_registry = DecoderRegistry::new(decoders);

    // Create and run ingestion service
    let ingestion = IngestionService::new(db, decoder_registry);
    ingestion.run(settings.mqtt).await
}
