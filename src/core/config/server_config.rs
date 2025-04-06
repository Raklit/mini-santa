use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub config_secret : String,
    pub log_path : String
}