use chrono::{DateTime, Utc};
use sensorvault_core::models::{Sensor, SensorData};
use sqlx::FromRow;
use sqlx::types::Uuid;
use std::fmt::Display;

#[derive(FromRow)]
pub struct DevicePg {
    pub id: Uuid,
    pub material_no: Option<String>,
    pub serial_no: Option<String>,
    pub custom_id: Option<String>,
    pub name: String,
    pub device_type: Option<String>,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct SensorPg {
    pub id: String,
    pub device_id: Option<Uuid>,
    pub channel: String,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Into<Sensor> for SensorPg {
    fn into(self) -> Sensor {
        Sensor {
            id: self.id,
            device_id: self.device_id,
            channel: self.channel,
            unit: self.unit,
            description: self.description,
            created_at: self.created_at,
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct SensorDataPg {
    pub time: DateTime<Utc>,
    pub sensor_id: String,
    pub value: f64,
}

impl Into<SensorData> for SensorDataPg {
    fn into(self) -> SensorData {
        SensorData {
            time: self.time,
            sensor_id: self.sensor_id,
            value: self.value,
        }
    }
}

impl Display for SensorDataPg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}] id={} value={}",
            self.time.format("%d.%m.%Y %H:%M:%S"),
            self.sensor_id,
            self.value
        ))
    }
}

#[cfg(test)]
mod tests {
    use sqlx::types::chrono::Utc;
    use super::*;

    #[test]
    fn test_db_sensor_into_model() {
        let now = Utc::now();
        let device_id = Uuid::new_v4();
        let db_sensor = SensorPg {
            id: "temperature/123".to_string(),
            device_id: Some(device_id),
            channel: "temperature".to_string(),
            unit: Some("°C".to_string()),
            description: Some("some temperature sensor".to_string()),
            created_at: now,
        };
        let sensor: Sensor = db_sensor.into();
        assert_eq!(sensor.id, "temperature/123".to_string());
        assert_eq!(sensor.device_id.unwrap(), device_id);
        assert_eq!(sensor.channel, "temperature".to_string());
        assert_eq!(sensor.unit, Some("°C".to_string()));
        assert_eq!(sensor.description, Some("some temperature sensor".to_string()));
        assert_eq!(sensor.created_at, now);
    }

    #[test]
    fn test_db_sensor_data_into_model() {
        let now = Utc::now();
        let db_data = SensorDataPg {
            time: now,
            sensor_id: "test-1".to_string(),
            value: 23.45,
        };
        let sensor_data: SensorData = db_data.into();
        assert_eq!(sensor_data.time, now);
        assert_eq!(sensor_data.sensor_id, "test-1");
        assert_eq!(sensor_data.value, 23.45);
    }
}