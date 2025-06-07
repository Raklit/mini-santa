use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AuthConfig {
    pub auth_token_lifetime : u64,
    pub refresh_token_lifetime : u64,
    pub auth_code_lifetime : u64,
    pub check_session_status_freq : u64
}