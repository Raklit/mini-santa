use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AdminConfig {
    pub login : String,
    pub password : String,
    pub nickname : String,
    pub email : String
}