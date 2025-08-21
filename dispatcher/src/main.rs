use std::env;
use async_nats::jetstream;
use futures::StreamExt;
use common::SenMLRecord;
use sensor_store;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let db = sensor_store::connect().await.expect("cannot connect to database");

    // Connect to the NATS server
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    println!("Connecting to NATS at {}", nats_url);
    let client = async_nats::connect(nats_url).await?;
    let jet_stream = jetstream::new(client);
    let stream_name = String::from("TEMPERATURE_SENSOR_EVENTS");

    // First we create a stream and bind to it.
    let stream = jet_stream
        .create_stream(jetstream::stream::Config {
            name: stream_name,
            subjects: vec!["temp.>".into()],
            //subjects: vec!["events".into()],
            ..Default::default()
        }).await?;

    // Then, on that `Stream` use method to create Consumer and bind to it too.
    let consumer = stream.create_consumer(jetstream::consumer::pull::Config {
        durable_name: Some("consumer".into()),
        ..Default::default()
    }).await?;

    // Attach to the messages iterator for the Consumer.
    // The iterator does its best to optimize retrieval of messages from the server.
    let mut messages = consumer.messages().await?; // we can also add .take(10) so the stream is closed after receiving 10 messages

    // Iterate over messages.
    println!("waiting for messages...");
    while let Some(message) = messages.next().await {
        let message = message?;

        // parse sensor_id as last part of subject
        let sensor_id_str = message.subject.split('.').last().expect("cannot parse sensor_id");
        let sensor_id = sensor_id_str.parse::<i32>().expect("cannot parse sensor_id");
        println!("received measurement from sensor #{}", sensor_id);

        // Deserialize from CBOR - also support JSON as a fallback
        let record: SenMLRecord = serde_cbor::from_slice(&message.payload)
            .or_else(|_| serde_json::from_slice(&message.payload))
            .expect("error at CBOR or JSON deserialization");
        println!("{}", record);

        sensor_store::save_sensor_data(&db, record, sensor_id).await.expect("error while saving sensor data");

        // acknowledge the message
        message.ack().await?;
    }

    Ok(())
}
