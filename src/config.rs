use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::with_name("config").required(false))
        .add_source(Environment::with_prefix("APP"))
        .build()?;

    config.try_deserialize()
}
