use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub config_secret : String,
    pub log_path : String
}