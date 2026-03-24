use crate::ingestion::decoder::{DecodedSensorReading, RawInput, SensorDataDecoder};
use anyhow::Context;
use std::str::from_utf8;

const CHANNEL_UNITS: &[(&str, &str)] = &[
    ("temperature", "°C"),
    ("humidity", "%"),
    ("pressure", "hPa"),
    ("co2", "ppm"),
    ("illuminance", "lx"),
    ("battery", "%"),
    ("voltage", "V"),
    ("motion", "bool"),
];

pub struct RawMQTTDecoder;

impl SensorDataDecoder for RawMQTTDecoder {
    fn matches(&self, input: &RawInput) -> bool {
        matches!(
            input,
            // TODO: be more explicit in what can/cannot be decoded!
            RawInput::Mqtt { topic, .. } if !topic.starts_with("zigbee2mqtt/")
        )
    }

    fn decode(&self, input: &RawInput) -> anyhow::Result<Vec<DecodedSensorReading>> {
        let RawInput::Mqtt { topic, payload } = input else {
            anyhow::bail!("RawMqttAdapter only handles RawInput::Mqtt");
        };

        let value = from_utf8(payload)
            .context("payload is not valid UTF-8")?
            .trim()
            .parse::<f64>()
            .context("payload is not a valid float")?;

        let (channel, unit) = match channel_and_unit_from_topic(topic) {
            Some((channel, unit)) => (channel.to_string(), Some(unit.to_string())),
            None => ("unknown".to_string(), None),
        };
        Ok(vec![DecodedSensorReading {
            id: topic.to_string(),
            channel,
            unit,
            value,
        }])
    }
}

fn channel_and_unit_from_topic(topic: &str) -> Option<(&'static str, &'static str)> {
    let segments: Vec<&str> = topic.split('/').collect();
    CHANNEL_UNITS
        .iter()
        .find(|(channel, _)| segments.contains(channel))
        .map(|(channel, unit)| (*channel, *unit))
}
