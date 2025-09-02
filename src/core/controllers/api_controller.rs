use axum::{body::Body, extract::{Path, State}, http::{HeaderMap, HeaderValue, Request, StatusCode}, middleware::{from_fn_with_state, Next}, response::IntoResponse, routing::{get, post}, Json, Router};
use axum_auth::AuthBearer;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{core::{controllers::{get_current_user_id, get_current_user_nickname, sign_in, sign_up}, data_model::traits::{IAccountRelated, IAccountSession}, services::get_access_by_access_token}, AppState};

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

pub async fn check_auth(State(state) : State<AppState>, AuthBearer(access_token) : AuthBearer, mut request : Request<Body>, next : Next) -> impl IntoResponse {
    let now_time = Utc::now();

    if access_token.is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "need auth token").into_response()); 
    }
    let account_session_option  = get_access_by_access_token(&access_token, &state).await;
    if account_session_option.is_none() {
        return Err((StatusCode::FORBIDDEN, "token doesn't exists").into_response()); 
    }
    let account_session = account_session_option.unwrap();
    let access_token_lifetime = state.config.lock().await.auth.access_token_lifetime;
    
    let lifetime_end = account_session.access_token_creation_date() + Duration::seconds(access_token_lifetime.try_into().unwrap());
    if lifetime_end < now_time {
        return Err((StatusCode::UNAUTHORIZED, "access token is expired").into_response());
    }

    while request.headers_mut().contains_key("account_id") {
        request.headers_mut().remove("account_id");
    }
    request.headers_mut().append("account_id", HeaderValue::from_str(account_session.account_id()).unwrap());
    return Ok(next.run(request).await);

}