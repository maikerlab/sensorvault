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
