use axum::{body::Body, extract::State, http::{HeaderName, HeaderValue, Request, StatusCode}, middleware::Next, response::IntoResponse, Json};
use axum_auth::AuthBearer;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{core::{data_model::traits::{IAccountRelated, IAccountSession}, services::get_access_by_access_token}, AppState};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ApiResponseStatus {
    OK = 0,
    WARNING = 1,
    ERROR = 2
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub status : ApiResponseStatus,
    pub body : serde_json::Value
}

impl ApiResponse {
    pub fn new(status : ApiResponseStatus, body : serde_json::Value) -> Self {
        return ApiResponse {
            status : status,
            body : body
        };
    }

    pub fn error_from_str(err_msg : &str) -> Self {
        let err_msg_string = String::from(err_msg);
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg_string).unwrap());
    }

    pub fn is_ok(&self) -> bool {
        return self.status.eq(&ApiResponseStatus::OK);
    }
}

// easy routes

pub async fn hello() -> String {
    return String::from("Hello World!");
}

pub async fn ping() -> String {
    return String::from("pong");
}

// auth middlware

#[derive(Serialize, Deserialize, Clone)]
pub struct OAuth2ErrorResponse {
    pub error : String,
    pub error_description : String
}

impl OAuth2ErrorResponse {
    pub fn new(error : &str, error_description : &str) -> Self {
        return OAuth2ErrorResponse { error: String::from(error), error_description: String::from(error_description) };
    }

    pub fn as_header_string(&self) -> String {
        let error = self.error.as_str();
        let error_description = self.error_description.as_str();
        let result = format!("Bearer error=\"${error}\", error_description=\"{error_description}\"");
        return result;
    }

    pub fn access_token_expired() -> Self {
        return Self::new("invalid_token", "The access token expired");
    }

    pub fn refresh_token_expired() -> Self {
        return Self::new("invalid_token", "The refresh token expired");
    }

    pub fn access_token_not_found() -> Self {
        return Self::new("invalid_token", "The access token not found");
    }

    pub fn refresh_token_not_found() -> Self {
        return Self::new("invalid_token", "The refresh token not found");
    }

    pub fn access_token_is_missing() -> Self {
        return Self::new("invalid_token", "The access token is missing");
    }

}

pub fn add_header(request : &mut Request<Body>, name : &str, content : &str) -> () {
    while request.headers().contains_key(name) {
        request.headers_mut().remove(name);
    }
    request.headers_mut().insert(
        HeaderName::from_bytes(name.as_bytes()).unwrap(),
        HeaderValue::from_str(content).unwrap(),
    );
}

pub async fn check_auth(State(state) : State<AppState>, AuthBearer(access_token) : AuthBearer, mut request : Request<Body>, next : Next) -> impl IntoResponse {
    let now_time = Utc::now();

    if access_token.is_empty() {
        let err = OAuth2ErrorResponse::access_token_is_missing();
        let err_msg = err.as_header_string();
        add_header(&mut request, "Content-Type", "application/json");
        add_header(&mut request, "WWW-Authentificate", err_msg.as_str());
        return Err((StatusCode::UNAUTHORIZED, Json(err)).into_response()); 
    }
    let account_session_option  = get_access_by_access_token(&access_token, &state).await;
    if account_session_option.is_none() {
        let err = OAuth2ErrorResponse::access_token_not_found();
        let err_msg = err.as_header_string();
        add_header(&mut request, "Content-Type", "application/json");
        add_header(&mut request, "WWW-Authentificate", err_msg.as_str());
        return Err((StatusCode::FORBIDDEN, Json(err)).into_response()); 
    }
    let account_session = account_session_option.unwrap();
    let access_token_lifetime = state.config.lock().await.auth.access_token_lifetime;
    
    let lifetime_end = account_session.access_token_creation_date() + Duration::seconds(access_token_lifetime.try_into().unwrap());
    if lifetime_end < now_time {
        let err = OAuth2ErrorResponse::access_token_expired();
        let err_msg = err.as_header_string();
        add_header(&mut request, "Content-Type", "application/json");
        add_header(&mut request, "WWW-Authentificate", err_msg.as_str());
        return Err((StatusCode::UNAUTHORIZED, Json(err)).into_response());
    }

    add_header(&mut request, "account_id", account_session.account_id());
    return Ok(next.run(request).await);

}