use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub db_file : String,
    pub connection_number : u64
}