use sqlx::{FromRow, PgPool};

#[derive(FromRow)]
pub struct Sensor {
    pub id: i32,
    pub r#type:  Option<String>,
    pub location: Option<String>,
    // pub name: String,
    // pub type_id: Uuid,
    // pub public_key: Vec<u8>,
    // pub status: String,
    // pub registered_at: NaiveDateTime,
}

pub async fn register_sensor(
    pool: &PgPool,
    id: i32,
    sensor_type: &str,
    location: &str,
) -> Result<i64, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO sensors (id, type, location)
        VALUES ($1, $2, $3)
        ON CONFLICT (id) DO NOTHING
        "#,
        id,
        sensor_type,
        location,
    )
        .execute(pool)
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT * FROM sensors WHERE id = $1")
        .bind(id)
        .bind(123)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn get_sensor_by_id(pool: &PgPool, id: i32) -> Result<Sensor, sqlx::Error> {
    let sensor = sqlx::query_as!(
        Sensor,
        r#"SELECT * FROM sensors WHERE id = $1"#,
        id,
    )
        .fetch_one(pool)
        .await?;
    Ok(sensor)
}