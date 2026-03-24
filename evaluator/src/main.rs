use crate::model::{Features, SensorHealthModel};
use std::path::Path;

mod model;

fn main() {
    let onnx_path = Path::new("evaluator/onnx/sensor_health.onnx");

    // Normal reading
    let normal = Features {
        value: 21.3,
        mean: 21.1,
        std_dev: 1.2,
        min: 19.5,
        max: 23.0,
        slope: 0.01,
        z_score: 0.17,
    };

    let model = SensorHealthModel::load(onnx_path).expect("failed to load sensor health model");
    match model.predict(&normal) {
        Ok(result) => {
            println!(
                "Normal  → health: {}, anomaly: {}",
                result.health_score, result.is_anomaly
            );
        }
        Err(err) => {
            println!("Failed to run inference: {}", err);
        }
    }
}
