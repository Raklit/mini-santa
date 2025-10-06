use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AuthConfig {
    pub access_token_lifetime : u64,
    pub refresh_token_lifetime : u64,
    pub auth_code_lifetime : u64,
    pub check_session_status_freq : u64,
    pub check_auth_code_status_freq : u64,
    pub oauth2_client_secret : String
}