use std::collections::HashMap;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Form, Json};
use serde::{Deserialize, Serialize};

use crate::{core::{data_model::{implementations::AccountSession, traits::IAccountSession}, services::{sign_in_by_auth_code, sign_in_by_refresh_token, sign_in_by_user_creditials, user_sign_up, SignUpStatus}}, AppState};

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

#[derive(Deserialize)]
pub struct SignInData {
    pub grant_type : String,
    pub client_id : String,
    pub client_secret : String,
    pub username : Option<String>,
    pub password : Option<String>,
    pub code : Option<String>,
    pub refresh_token : Option<String>,
    pub redirect_uri : Option<String>
}

pub async fn sign_in(State(state) : State<AppState>, Form(sign_in_data) : Form<SignInData>) -> impl IntoResponse {
    let grant_type = sign_in_data.grant_type.as_str();
    let client_id = sign_in_data.client_id.as_str();
    let client_secret = sign_in_data.client_secret.as_str();
    let mut account_session_option : Option<AccountSession> = None;
    if grant_type == "password" {
        let username_string = sign_in_data.username.unwrap_or(String::new());
        let username = username_string.as_str();
        let password_string = sign_in_data.password.unwrap_or(String::new());
        let password = password_string.as_str();
        account_session_option = transform_account_session(sign_in_by_user_creditials(username, password, client_id, client_secret, &state).await);
    } else if grant_type == "refresh_token" {
        let refresh_token_string = sign_in_data.refresh_token.unwrap_or(String::new());
        let refresh_token = refresh_token_string.as_str();
        account_session_option = transform_account_session(sign_in_by_refresh_token(refresh_token, client_id, client_secret, &state).await);
    } else if grant_type == "code" {
        let code_string = sign_in_data.code.unwrap_or(String::new());
        let code = code_string.as_str();
        account_session_option = transform_account_session(sign_in_by_auth_code(code, client_id, client_secret, &state).await);
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


#[derive(Deserialize)]
pub struct SignUpData {
    pub login : String,
    pub password : String,
    pub confirm_password : String,
    pub nickname : String,
    pub email : String
}

/// TODO: ALMOST IMPLEMENTET (check validation data function)
pub async fn sign_up(State(state) : State<AppState>, Form(sign_up_data) : Form<SignUpData>) -> impl IntoResponse {

    let login = sign_up_data.login.as_str();
    let password = sign_up_data.password.as_str();
    let confirm_password = sign_up_data.confirm_password.as_str();
    let nickname = sign_up_data.nickname.as_str();
    let email = sign_up_data.email.as_str();


    let result = user_sign_up(login, password, confirm_password, nickname, email, &state).await;
    let data_valid = result.clone().into_iter().all(|s : SignUpStatus| -> bool { s == SignUpStatus::OK });
    if !data_valid {
        return Err((StatusCode::BAD_REQUEST, "").into_response());
    }
    return Ok((StatusCode::OK, "").into_response());
}