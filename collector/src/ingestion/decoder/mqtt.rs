use crate::ingestion::decoder::{DecodedSensorReading, RawInput, SensorDataDecoder};
use anyhow::Context;
use serde_json::Value;
use std::str::from_utf8;
use tracing::{debug, info};

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

        let value_str = from_utf8(payload)
            .context("payload is not valid UTF-8")?
            .trim();
        debug!("Decoding RawMQTT payload: {}", value_str);
        let value = value_str
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

pub struct Zigbee2MQTTDecoder;
impl SensorDataDecoder for Zigbee2MQTTDecoder {
    fn matches(&self, input: &RawInput) -> bool {
        matches!(
            input,
            RawInput::Mqtt { topic, .. } if topic.starts_with("zigbee2mqtt/")
        )
    }

    fn decode(&self, input: &RawInput) -> anyhow::Result<Vec<DecodedSensorReading>> {
        let RawInput::Mqtt { topic, payload } = input else {
            anyhow::bail!("RawMqttAdapter only handles RawInput::Mqtt");
        };
        let payload_str = from_utf8(payload)?;
        let payload_json = serde_json::from_str::<Value>(payload_str)?;
        debug!("Decoding Zigbee2MQTT payload: {:?}", payload_json);
        Ok(zigbee2mqtt_to_sensor_reading(
            topic.to_string(),
            payload_json,
        ))
    }
}

fn zigbee2mqtt_to_sensor_reading(topic: String, payload: Value) -> Vec<DecodedSensorReading> {
    let mut readings: Vec<DecodedSensorReading> = vec![];
    if let Some(state) = payload.get("state").and_then(Value::as_str) {
        info!("Plug state: {}", state);
    }
    if let Some(temperature) = payload.get("temperature").and_then(Value::as_f64) {
        info!("Temperature: {:1} °C", temperature);
        readings.push(DecodedSensorReading {
            id: topic.clone(),
            channel: "temperature".to_string(),
            value: temperature,
            unit: Some("°C".to_string()),
        })
    }
    if let Some(humidity) = payload.get("humidity").and_then(Value::as_f64) {
        info!("Humidity: {:1} %RF", humidity);
        readings.push(DecodedSensorReading {
            id: topic,
            channel: "humidity".to_string(),
            value: humidity,
            unit: Some("%RF".to_string()),
        })
    }
    vec![]
}

fn channel_and_unit_from_topic(topic: &str) -> Option<(&'static str, &'static str)> {
    let segments: Vec<&str> = topic.split('/').collect();
    CHANNEL_UNITS
        .iter()
        .find(|(channel, _)| segments.contains(channel))
        .map(|(channel, unit)| (*channel, *unit))
}
