use crate::persistence::models::{SensorDataPg, SensorPg};
use crate::persistence::{SensorDataRepository, SensorRepository};
use core::models::SensorData;
use core::models::{CreateSensor, CreateSensorData, Sensor};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub struct PostgresDatabase {
    pool: PgPool,
}

impl PostgresDatabase {
    pub async fn connect(connection_url: String, max_connections: u32) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(connection_url.as_str())
            .await?;
        Ok(Self { pool })
    }
}

impl SensorRepository for PostgresDatabase {
    async fn find_sensor_by_id(&self, id: &str) -> anyhow::Result<Option<Sensor>> {
        let sensor: Option<SensorPg> = sqlx::query_as(
            r#"
                SELECT id, device_id, channel, unit, description, created_at
                FROM sensors
                WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(sensor.map(|s| s.into()))
    }

    async fn save_sensor(&self, sensor: CreateSensor) -> anyhow::Result<Sensor> {
        let sensor: SensorPg = sqlx::query_as(
            r#"
                INSERT INTO sensors (id, channel, unit, description)
                VALUES ($1, $2, $3, $4)
                RETURNING id, device_id, channel, unit, description, created_at
            "#,
        )
        .bind(sensor.id)
        .bind(sensor.channel)
        .bind(sensor.unit)
        .bind(sensor.description)
        .fetch_one(&self.pool)
        .await?;
        Ok(sensor.into())
    }
}

impl SensorDataRepository for PostgresDatabase {
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
        Ok(sensor_data.into())
    }
}
