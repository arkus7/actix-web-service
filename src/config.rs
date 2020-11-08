use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: i32,
    pub database_name: String,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub db: DatabaseConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
