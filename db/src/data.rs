use chrono::{DateTime, Utc};
use sqlx::PgPool;
use common::SenMLRecord;
use anyhow::Result;

#[derive(sqlx::FromRow, Debug)]
pub struct SensorData {
    pub timestamp: DateTime<Utc>,
    pub sensor_id: Option<i32>,
    pub value: Option<f64>,
    pub unit: Option<f64>,
}

pub async fn save_record(pool: &PgPool, record: SenMLRecord, sensor_id: i32) -> Result<()> {
    let ts = chrono::DateTime::from_timestamp(record.timestamp.unwrap_or(chrono::Utc::now().timestamp()), 0);
    sqlx::query!(
        r#"
        INSERT INTO sensor_data (time, sensor_id, value, unit)
        VALUES ($1, $2, $3, $4)
        "#,
        ts,
        sensor_id,
        record.value,
        record.unit
    )
        .execute(pool)
        .await?;

    Ok(())
}
