use async_nats::jetstream;
use async_nats::jetstream::consumer::Consumer;
use async_nats::jetstream::consumer::pull::Config;
use async_nats::jetstream::Context;
use anyhow::Result;
use tracing::{info};

pub async fn connect_nats(url: String) -> Result<Context> {
    info!("Connecting to NATS at {}", url);
    let client = async_nats::connect(url).await?;
    let jet_stream = jetstream::new(client);
    Ok(jet_stream)
}

pub async fn subscribe(nats: &Context, stream_name: String, subjects: Vec<String>) -> Result<Consumer<Config>> {
    // First we create a stream and bind to it.
    let stream = nats
        .create_stream(jetstream::stream::Config {
            name: stream_name,
            subjects,
            //subjects: vec!["events".into()],
            ..Default::default()
        }).await?;

    // Then, on that `Stream` use method to create Consumer and bind to it too.
    let consumer = stream.create_consumer(Config {
        durable_name: Some("consumer".into()),
        ..Default::default()
    }).await?;
    Ok(consumer)
}