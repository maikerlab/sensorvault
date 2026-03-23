use core::models::CreateSensor;
use core::models::CreateSensorData;
use core::models::Sensor;
use core::models::SensorData;

pub mod postgres;
mod models;

pub trait SensorRepository {
    fn find_sensor_by_id(&self, topic: &str) -> impl Future<Output = anyhow::Result<Option<Sensor>>> + Send;
    fn save_sensor(&self, sensor: CreateSensor) -> impl Future<Output = anyhow::Result<Sensor>> + Send;
}

pub trait SensorDataRepository {
    fn save_sensor_reading(&self, reading: &CreateSensorData) -> impl Future<Output = anyhow::Result<SensorData>> + Send;
}

