use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub db_file : String,
    pub connection_number : u64
}