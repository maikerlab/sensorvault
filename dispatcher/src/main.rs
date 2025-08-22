use common::SenMLRecord;
use futures::StreamExt;
use db::data::save_record;
use tracing::info;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // init logging
    tracing_subscriber::fmt::init();

    let db = db::connect_postgres().await.expect("cannot connect to database");
    let nats = messaging::connect_nats().await.expect("cannot connect to NATS JetStream");

    let stream_name = String::from("TEMPERATURE_SENSOR_EVENTS");
    let subjects = vec!["temp.>".into()];
    let consumer = messaging::subscribe(&nats, stream_name, subjects).await.expect("cannot subscribe to NATS JetStream");
    // Attach to the messages iterator for the Consumer.
    let mut messages = consumer.messages().await?;

    // Iterate over messages.
    info!("waiting for messages from workers...");
    while let Some(message) = messages.next().await {
        let message = message?;

        // parse sensor_id as last part of subject
        let sensor_id_str = message.subject.split('.').last().expect("cannot parse sensor_id");
        let sensor_id = sensor_id_str.parse::<i32>().expect("cannot parse sensor_id");

        // Deserialize from CBOR - also support JSON as a fallback
        let record: SenMLRecord = serde_cbor::from_slice(&message.payload)
            .or_else(|_| serde_json::from_slice(&message.payload))
            .expect("error at CBOR or JSON deserialization");
        info!("Received SenMLRecord: {}", record);

        save_record(&db, record, sensor_id).await.expect("error while saving sensor data");
        info!("saved sensor data");

        // acknowledge the message
        message.ack().await.map_err(|e| anyhow::anyhow!(e))?;
    }

    Ok(())
}
