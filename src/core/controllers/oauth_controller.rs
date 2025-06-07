use std::collections::HashMap;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use axum_extra::TypedHeader;
use serde::{Deserialize, Serialize};

use crate::{core::{data_model::{implementations::AccountSession, traits::IAccountSession}, services::{sign_in_by_refresh_token, sign_in_by_user_creditials}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token : String,
    pub refresh_token : String,
    pub token_type : String,
    pub expires_in : u64,
    pub scope : String
}

#[derive(Serialize, Deserialize)]
pub struct AuthenticateHeader {
    pub realm : String,
    pub charset : String,
    pub error : String,
    pub error_description : String
}

fn transform_account_session(account_session_option : Option<impl IAccountSession>) -> Option<AccountSession> {
    if account_session_option.is_none() { return None; }
    let account_session = account_session_option.unwrap();
    let result = AccountSession {
         id: String::from(account_session.id()),
         account_id: String::from(account_session.account_id()),
         auth_token: String::from(account_session.auth_token()), 
         refresh_token: String::from(account_session.refresh_token()), 
         start_date: account_session.start_date(), 
         auth_token_creation_date: account_session.auth_token_creation_date(), 
         refresh_token_creation_date: account_session.refresh_token_creation_date(), 
         last_usage_date: account_session.last_usage_date()
    };
    return Some(result);
}


async fn sign_in(params: Query<HashMap<String, String>>, State(state) : State<AppState>) -> impl IntoResponse {
    let grant_type = params.get("grant_type").map_or("", |v| v);
    let client_id = params.get("client_id").map_or("", |v| v);
    let client_secret = params.get("client_secret").map_or("", |v| v);
    let mut account_session_option : Option<AccountSession> = None;
    if grant_type == "password" {
        let username = params.get("username").map_or("", |v| v);
        let password = params.get("password").map_or("", |v| v);
        account_session_option = transform_account_session(sign_in_by_user_creditials(username, password, client_id, client_secret, &state).await);
    } else if grant_type == "refresh_token" {
        let refresh_token = params.get("refresh_token").map_or("", |v| v);
        account_session_option = transform_account_session(sign_in_by_refresh_token(refresh_token, client_id, client_secret, &state).await);
    }

    if account_session_option.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "wrong data").into_response()); 
    }

    let account_session = account_session_option.unwrap();
    let response = AuthResponse {
        access_token: String::from(account_session.auth_token()),
        refresh_token: String::from(account_session.refresh_token()),
        token_type: String::from("Bearer"),
        expires_in: state.config.lock().await.auth.auth_token_lifetime,
        scope: String::from("read+write")
    };

    return Ok((StatusCode::OK, Json(response)).into_response());
}

pub fn auth_router() -> Router<AppState> {
    return Router::new()
    .route("/token", get(sign_in))
}