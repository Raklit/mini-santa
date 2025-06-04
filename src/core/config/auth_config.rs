use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthConfig {
    pub auth_token_lifetime : u64,
    pub refresh_token_lifetime : u64,
    pub auth_code_lifetime : u64
}