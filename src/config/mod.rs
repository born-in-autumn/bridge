use std::sync::LazyLock;
use config::{Config, ConfigError};
use serde::Deserialize;
use crate::config::server::ServerConfig;
use database::DatabaseConfig;

mod server;
mod database;

static CONFIG:LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load().expect("Failed to init Config"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub dev: DevConfig,
    pub database: DatabaseConfig,
}
#[derive(Debug, Deserialize, Clone)]
pub struct DevConfig {
    pub default_redirect_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(config::FileFormat::Yaml)
                    .required(true)
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(",")
            )
            .build()?;
        config.try_deserialize()
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}