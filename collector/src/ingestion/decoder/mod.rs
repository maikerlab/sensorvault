pub mod raw;

use std::fmt::{Display, Formatter};
use crate::ingestion::input::RawInput;
use tracing::log::warn;

#[derive(Debug)]
pub struct DecodedSensorReading {
    pub channel: String,
    pub value: f64,
    pub unit: Option<String>,
}

impl Display for DecodedSensorReading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {:?}", self.channel, self.value, self.unit)
    }
}

pub trait SensorDataDecoder {
    /// Can this decoder handle this data source?
    /// Called before decode() – routing decision
    fn matches(&self, input: &RawInput) -> bool;

    /// Decode a raw input into one or more readings
    /// Returns Vec because some formats (SenML, Zigbee) carry multiple values
    fn decode(&self, input: &RawInput) -> anyhow::Result<Vec<DecodedSensorReading>>;
}

pub struct DecoderRegistry {
    decoders: Vec<Box<dyn SensorDataDecoder>>,
}

impl DecoderRegistry {
    pub fn new(decoders: Vec<Box<dyn SensorDataDecoder>>) -> DecoderRegistry {
        Self { decoders }
    }

    pub fn decode(&self, input: &RawInput) -> anyhow::Result<Vec<DecodedSensorReading>> {
        let decoder = self
            .decoders
            .iter()
            .find(|decoder| decoder.matches(input))
            .ok_or_else(|| {
                let label = match input {
                    RawInput::Mqtt { topic, .. } => format!("MQTT topic '{topic}'"),
                    RawInput::Manual {
                        material_no,
                        serial_no,
                        ..
                    } => format!("manual input '{material_no}/{serial_no}'"),
                };
                warn!("No decoder was found");
                anyhow::anyhow!("No decoder matches found => {}", label)
            })?;
        decoder.decode(input)
    }
}
