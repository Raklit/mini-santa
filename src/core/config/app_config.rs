use serde::Deserialize;

use super::server_config::ServerConfig;
use super::database_config::DatabaseConfig;

#[derive(Deserialize)]
pub struct AppConfig {
    pub server : ServerConfig,
    pub database : DatabaseConfig
}