use std::str::FromStr;

use chrono::prelude::*;
use sqlx::{Row, Executor};
use sqlx::sqlite::SqliteRow;
use tracing_subscriber::filter::combinator::Not;

use crate::core::data_model::implementations::AccountSession;
use crate::core::data_model::traits::IAccountSession;
use crate::core::functions::{execute_script_template_wo_return, generate_random_token, render_query_template};
use crate::AppState;

fn row_to_account_session(row : &SqliteRow) -> AccountSession {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let auth_token : &str = row.get("auth_token");
    let refresh_token : &str = row.get("refresh_token");
    let is_active : bool = row.get("is_active");
    let is_ended : bool = row.get("is_ended");

    let start_date_str : &str = row.get("start_date");
    let auth_token_creation_date_str : &str = row.get("auth_token_creation_date");
    let refresh_token_creation_date_str : &str = row.get("refresh_token_creation_date");
    let last_usage_date_str : &str = row.get("last_usage_date");

    return AccountSession { 
        id: String::from(id),
        account_id: String::from(account_id),
        auth_token: String::from(auth_token), 
        refresh_token: String::from(refresh_token),
        is_active: is_active,
        is_ended: is_ended, 
        start_date: DateTime::from_str(&start_date_str).unwrap(), 
        auth_token_creation_date: DateTime::from_str(&auth_token_creation_date_str).unwrap(),
        refresh_token_creation_date: DateTime::from_str(&refresh_token_creation_date_str).unwrap(),
        last_usage_date: DateTime::from_str(&last_usage_date_str).unwrap() 
    };
}

pub async fn is_account_session_already_exists_by_id(id : &str, state : &AppState) -> bool {
    const EXISTS_ACCOUNT_SESSION_BY_ID_TEMPLATE : &str = "database_scripts/account_session/exists_account_session_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    let command = render_query_template(EXISTS_ACCOUNT_SESSION_BY_ID_TEMPLATE, &context, &state);
    let result = state.db.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn is_account_session_already_exists_by_token(token : &str, state : &AppState) -> bool {
    const EXISTS_ACCOUNT_SESSION_BY_TOKEN_TEMPLATE : &str = "database_scripts/account_session/exists_account_session_by_token.sql";
    let mut context = tera::Context::new();
    context.insert("token", &token);
    let command = render_query_template(EXISTS_ACCOUNT_SESSION_BY_TOKEN_TEMPLATE, &context, &state);
    let result = state.db.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn create_account_session(id : &str, account_id : &str, state : &AppState) -> () {
    let creation_date = Utc::now();
    
    let mut auth_token : String;
    loop {
        auth_token = generate_random_token();
        let is_account_session_already_exists =  is_account_session_already_exists_by_token(&auth_token.as_str(), &state).await;
        if !is_account_session_already_exists { break; }
    }

    let mut refresh_token : String;
    loop {
        refresh_token = generate_random_token();
        let is_account_session_already_exists =  is_account_session_already_exists_by_token(refresh_token.as_str(), &state).await;
        if !is_account_session_already_exists { break; }
    }


    let account_session = AccountSession {
        id : String::from(id),
        account_id: String::from(account_id),
        auth_token: auth_token,
        refresh_token: refresh_token,
        is_active: true,
        is_ended: false,
        start_date: creation_date.clone(),
        auth_token_creation_date: creation_date.clone(),
        refresh_token_creation_date: creation_date.clone(),
        last_usage_date: creation_date.clone(),
     };
 
     let mut context = tera::Context::new();
     context.insert("id", &account_session.id.as_str());
     context.insert("account_id", &account_session.account_id.as_str());
     context.insert("auth_token", &account_session.auth_token.as_str());
     context.insert("refresh_token", &account_session.refresh_token.as_str());
     context.insert("is_active", &account_session.is_active);
     context.insert("is_ended", &account_session.is_ended);
     context.insert("start_date", &account_session.start_date.to_rfc3339());
     context.insert("auth_token_creation_date", &account_session.auth_token_creation_date.to_rfc3339());
     context.insert("refresh_token_creation_date", &account_session.refresh_token_creation_date.to_rfc3339());
     context.insert("last_usage_date", &account_session.last_usage_date.to_rfc3339());

     const CREATE_ACCOUNT_SESSION_TEMPLATE : &str = "database_scripts/account_session/create_account_session.sql";
     execute_script_template_wo_return(CREATE_ACCOUNT_SESSION_TEMPLATE, &context, &state).await;
 }

pub async fn get_account_session_by_id(id : &str, state : &AppState) -> Option<impl IAccountSession> {
    const GET_ACCOUNT_SESSION_BY_ID_TEMPLATE : &str = "database_scripts/account_session/get_account_session_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(GET_ACCOUNT_SESSION_BY_ID_TEMPLATE, &context, &state);
    let result = match state.db.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_account_session(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn get_account_session_by_token(token : &str, state : &AppState) -> Option<impl IAccountSession> {
    const GET_ACCOUNT_SESSION_BY_TOKEN_TEMPLATE : &str = "database_scripts/account_session/get_account_session_by_token.sql";
    let mut context = tera::Context::new();
    context.insert("token", &token);
    
    let command = render_query_template(GET_ACCOUNT_SESSION_BY_TOKEN_TEMPLATE, &context, &state);
    let result = match state.db.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_account_session(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn update_account_session_auth_token_by_id(id : &str, auth_token : &str, state : &AppState) -> () {
    const UPDATE_ACCOUNT_SESSION_AUTH_TOKEN_TEMPLATE : &str = "database_scripts/account_session/update_account_session_auth_token.sql";
    let now_time = Utc::now();
    
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("auth_token", &auth_token);
    context.insert("now", &now_time.to_rfc3339());
    execute_script_template_wo_return(UPDATE_ACCOUNT_SESSION_AUTH_TOKEN_TEMPLATE, &context, &state).await;
}

pub async fn update_account_session_refresh_token_by_id(id : &str, refresh_token : &str, state : &AppState) -> () {
    const UPDATE_ACCOUNT_SESSION_REFRESH_TOKEN_TEMPLATE : &str = "database_scripts/account_session/update_account_session_refresh_token.sql";
    let now_time = Utc::now();
    
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("refresh_token", &refresh_token);
    context.insert("now", &now_time.to_rfc3339());
    execute_script_template_wo_return(UPDATE_ACCOUNT_SESSION_REFRESH_TOKEN_TEMPLATE, &context, &state).await;
}

pub async fn delete_account_session_by_id(id : &str, state : &AppState) -> impl IAccountSession {
    const DELETE_ACCOUNT_SESSION_BY_ID_TEMPLATE : &str = "database_scripts/account_session/delete_account_session_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(DELETE_ACCOUNT_SESSION_BY_ID_TEMPLATE, &context, &state);
    let result = state.db.fetch_one(command.as_str()).await.unwrap();
    
    return row_to_account_session(&result);
}