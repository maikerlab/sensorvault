mod models;

use crate::persistence::models::{SensorDataPg, SensorPg};
use core::models::SensorData;
use core::models::{CreateSensor, CreateSensorData, Sensor};
use core::settings::DatabaseSettings;
use infra::persistence::{SensorDataRepository, SensorRepository};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(config: DatabaseSettings) -> anyhow::Result<Self> {
        let url = config.url.as_str();
        let pool = PgPoolOptions::new().max_connections(3).connect(url).await?;
        Ok(Self { pool })
    }
}

impl SensorRepository for Database {
    async fn get_sensor_by_topic(&self, topic: &str) -> anyhow::Result<Option<Sensor>> {
        let sensor: Option<SensorPg> = sqlx::query_as(
            r#"
                SELECT id, device_id, channel, unit, description, created_at
                FROM sensors
                WHERE id = $1
            "#,
        )
        .bind(topic)
        .fetch_optional(&self.pool)
        .await?;
        Ok(sensor.map(|s| sensor_from_db(s)))
    }

    async fn save_sensor(&self, sensor: CreateSensor) -> anyhow::Result<Sensor> {
        let sensor: SensorPg = sqlx::query_as(
            r#"
                INSERT INTO sensors (id, channel, unit, description)
                VALUES ($1, $2, $3, $4)
                RETURNING id, device_id, channel, unit, description
            "#,
        )
        .bind(sensor.id)
        .bind(sensor.channel)
        .bind(sensor.unit)
        .bind(sensor.description)
        .fetch_one(&self.pool)
        .await?;
        Ok(sensor_from_db(sensor))
    }
}

impl SensorDataRepository for Database {
    async fn save_sensor_reading(&self, reading: &CreateSensorData) -> anyhow::Result<SensorData> {
        let sensor_data: SensorDataPg = sqlx::query_as(
            r#"
                INSERT INTO sensor_data (time, sensor_id, value)
                VALUES ($1, $2, $3)
                RETURNING time, sensor_id, value
            "#,
        )
        .bind(reading.time)
        .bind(reading.sensor_id.as_str())
        .bind(reading.value)
        .fetch_one(&self.pool)
        .await?;
        Ok(sensor_data_from_db(sensor_data))
    }
}

fn sensor_from_db(s: SensorPg) -> Sensor {
    Sensor {
        id: s.id,
        device_id: s.device_id,
        channel: s.channel,
        unit: s.unit,
        description: s.description,
        created_at: s.created_at,
    }
}

fn sensor_data_from_db(sd: SensorDataPg) -> SensorData {
    SensorData {
        time: sd.time,
        sensor_id: sd.sensor_id,
        value: sd.value,
    }
}
