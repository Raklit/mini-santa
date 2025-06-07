use serde::Deserialize;

use super::server_config::ServerConfig;
use super::database_config::DatabaseConfig;
use super::auth_config::AuthConfig;


#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub server : ServerConfig,
    pub database : DatabaseConfig,
    pub auth : AuthConfig
}