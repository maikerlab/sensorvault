use async_nats::jetstream::Context;
use chrono::Utc;
use common::SenMLRecord;
use rand::Rng;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;
use std::env;
use tokio::time;

async fn publish_measurement(context: &Context, sensor_id: i32, record: SenMLRecord) -> Result<(), async_nats::Error> {
    let payload = serde_cbor::to_vec(&record)?;
    context.publish(format!("temp.{}", sensor_id), payload.into())
        .await? // publish
        .await?; // await acknowledgement
    Ok(())
}

async fn handle_message(ctx: &Context, topic: &str, payload: &str) -> Result<(), async_nats::Error> {
    let sensor_type = topic.split('/').nth(1).unwrap();
    println!("Got value {:.1} (sensor type: {}) from topic {}", payload, sensor_type, topic);
    let sensor_id = topic.split('/').nth(2).unwrap();
    let sensor_id = sensor_id.parse::<i32>().unwrap();
    let value = payload.parse::<f64>().unwrap();
    let unit = "C";
    let msg = SenMLRecord {
        name: format!("{}-{}", sensor_type, sensor_id),
        value,
        unit: Some(unit.to_string()),
        timestamp: Some(Utc::now().timestamp())
    };
    publish_measurement(&ctx, sensor_id, msg).await?;
    println!("Message published successfully");
    Ok(())
}

async fn pub_demo(ctx: &Context) -> Result<(), async_nats::Error> {
    // just for demo - send 10 random values
    for _i in 0..10 {
        let mut rng = rand::rng();
        let sensor_id: i32 = rng.random_range(1..4);
        let value: f64 = rng.random_range(18.0..25.0);
        let value_rounded = (value * 10.0).round() / 10.0;
        let msg = SenMLRecord {
            name: format!("temp-{}", sensor_id),
            value: value_rounded,
            unit: Some("C".to_string()),
            timestamp: Some(Utc::now().timestamp())
        };
        publish_measurement(&ctx, sensor_id, msg).await?;
        println!("Message published successfully");
        time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    // Connect to the NATS server
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let client = async_nats::connect(nats_url).await?;
    let nats = async_nats::jetstream::new(client);

    let mut mqttoptions = MqttOptions::new("rumqtt-async", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("sensors/+/+", QoS::AtMostOnce).await.unwrap();

    println!("Waiting for MQTT messages...");
    while let Ok(event) = eventloop.poll().await {
        match event {
            Event::Incoming(Incoming::Publish(p)) => {
                println!("Received at {} = {:?}", p.topic.clone(), p.payload.clone());
                if let Ok(payload_str) = str::from_utf8(&p.payload) {
                    handle_message(&nats, p.topic.as_str(), payload_str).await?;
                }

            },
            _ => {}
        }
    }

    Ok(())
}
