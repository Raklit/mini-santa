use std::collections::HashMap;

use axum::{body::Body, extract::{Query, State}, http::{request, Request, StatusCode}, response::IntoResponse, Json};
use axum_extra::{headers, TypedHeader};
use serde::{Deserialize, Serialize};

use crate::{core::{data_model::{implementations::AccountSession, traits::IAccountSession}, functions::generate_random_token, services::{create_account, sign_in_by_refresh_token, sign_in_by_user_creditials, user_sign_up, SignUpStatus}}, AppState};

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
    
    let id = account_session.id();
    let account_id = account_session.account_id();
    let access_token = account_session.access_token();
    let refresh_token = account_session.refresh_token();
    let start_date = account_session.start_date();
    let access_token_creation_date = account_session.access_token_creation_date();
    let refresh_token_creation_date = account_session.refresh_token_creation_date();
    let last_usage_date = account_session.last_usage_date();
    
    let result = AccountSession::new(id, account_id, access_token, refresh_token, start_date, access_token_creation_date, refresh_token_creation_date, last_usage_date);
    return Some(result)
}

pub async fn sign_in(params: Query<HashMap<String, String>>, State(state) : State<AppState>) -> impl IntoResponse {
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
        access_token: String::from(account_session.access_token()),
        refresh_token: String::from(account_session.refresh_token()),
        token_type: String::from("Bearer"),
        expires_in: state.config.lock().await.auth.access_token_lifetime,
        scope: String::from("read+write")
    };

    return Ok((StatusCode::OK, Json(response)).into_response());
}


struct SignUpData {
    pub login : String,
    pub password : String,
    pub confirm_password : String,
    pub nickname : String,
    pub email : String
}

/// TODO: NOT IMPLEMENTET YET
pub async fn sign_up(State(state) : State<AppState>, request : Request<Body>) -> impl IntoResponse {
    let login = request.headers().get("login").unwrap().to_str().unwrap();
    let password = request.headers().get("password").unwrap().to_str().unwrap();
    let confirm_password = request.headers().get("confirm_password").unwrap().to_str().unwrap();
    let nickname= request.headers().get("nickname").unwrap().to_str().unwrap();
    let email= request.headers().get("email").unwrap().to_str().unwrap();
    

    let result = user_sign_up(login, password, confirm_password, nickname, email, &state).await;
    let data_valid = result.clone().into_iter().all(|s : SignUpStatus| -> bool { s == SignUpStatus::OK });
    if !data_valid {
        return Err((StatusCode::BAD_REQUEST, "").into_response());
    }
    return Ok((StatusCode::OK, "").into_response());
}