use config::FileFormat::Toml;
use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub database_url: String,
    pub nats_url: String,
    pub mqtt_host: Option<String>,
    pub mqtt_port: Option<u16>,
}

impl Settings {
    pub fn load() -> Self {
        let settings_file = File::new("settings.toml", Toml);
        let settings = Config::builder()
            .add_source(settings_file.required(true))
            .add_source(config::Environment::with_prefix("IOT"))
            .build()
            .expect("Failed to load settings");
        settings.try_deserialize().expect("Failed to parse settings")
    }
}