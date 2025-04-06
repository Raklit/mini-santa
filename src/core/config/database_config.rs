use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host : String,
    pub port : u64,
    pub db_name : String,
    pub user : String,
    pub password : String,
    pub connection_number : u64
}