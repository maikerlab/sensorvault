use serde::{Deserialize, Serialize};

pub enum RawInput {
    Mqtt {
        topic: String,
        payload: Vec<u8>,
    },
    Manual {
        material_no: String,
        serial_no: String,
        channel: String,
        value: f64,
        unit: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SenMLRecord {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "u", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "v")]
    pub value: f64,
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}