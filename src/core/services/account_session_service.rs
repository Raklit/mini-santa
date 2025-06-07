use std::str::FromStr;

use chrono::prelude::*;
use sqlx::{Row, Executor};
use sqlx::sqlite::SqliteRow;

use crate::core::data_model::implementations::AccountSession;
use crate::core::data_model::traits::IAccountSession;
use crate::core::functions::{execute_script_template_wo_return, render_query_template};
use crate::AppState;

fn row_to_account_session(row : &SqliteRow) -> AccountSession {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let auth_token : &str = row.get("auth_token");
    let refresh_token : &str = row.get("refresh_token");

    let start_date_str : &str = row.get("start_date");
    let auth_token_creation_date_str : &str = row.get("auth_token_creation_date");
    let refresh_token_creation_date_str : &str = row.get("refresh_token_creation_date");
    let last_usage_date_str : &str = row.get("last_usage_date");

    return AccountSession { 
        id: String::from(id),
        account_id: String::from(account_id),
        auth_token: String::from(auth_token), 
        refresh_token: String::from(refresh_token),
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
    let command = render_query_template(EXISTS_ACCOUNT_SESSION_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    
    return val == 1;
}

pub async fn is_account_session_already_exists_by_token(token : &str, state : &AppState) -> bool {
    const EXISTS_ACCOUNT_SESSION_BY_TOKEN_TEMPLATE : &str = "database_scripts/account_session/exists_account_session_by_token.sql";
    
    let mut context = tera::Context::new();
    context.insert("token", &token);
    
    let command = render_query_template(EXISTS_ACCOUNT_SESSION_BY_TOKEN_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    
    return val == 1;
}

pub async fn create_account_session(id : &str, account_id : &str, auth_token : &str, refresh_token : &str, state : &AppState) -> () {
    let creation_date = Utc::now().to_rfc3339();
 
     let mut context = tera::Context::new();
     context.insert("id", &id);
     context.insert("account_id", &account_id);
     context.insert("auth_token", &auth_token);
     context.insert("refresh_token", &refresh_token);
     context.insert("start_date", &creation_date);
     context.insert("auth_token_creation_date", &creation_date);
     context.insert("refresh_token_creation_date", &creation_date);
     context.insert("last_usage_date", &creation_date);

     const CREATE_ACCOUNT_SESSION_TEMPLATE : &str = "database_scripts/account_session/create_account_session.sql";
     execute_script_template_wo_return(CREATE_ACCOUNT_SESSION_TEMPLATE, &context, &state).await;
 }

pub async fn get_account_session_by_id(id : &str, state : &AppState) -> Option<impl IAccountSession> {
    const GET_ACCOUNT_SESSION_BY_ID_TEMPLATE : &str = "database_scripts/account_session/get_account_session_by_id.sql";
    
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(GET_ACCOUNT_SESSION_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_account_session(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn get_account_session_by_auth_token(auth_token : &str, state : &AppState) -> Option<impl IAccountSession> {
    const GET_ACCOUNT_SESSION_BY_AUTH_TOKEN_TEMPLATE : &str = "database_scripts/account_session/get_account_session_by_auth_token.sql";
    
    let mut context = tera::Context::new();
    context.insert("auth_token", &auth_token);
    
    let command = render_query_template(GET_ACCOUNT_SESSION_BY_AUTH_TOKEN_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_account_session(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn get_account_session_by_refresh_token(refresh_token : &str, state : &AppState) -> Option<impl IAccountSession> {
    const GET_ACCOUNT_SESSION_BY_REFRESH_TOKEN_TEMPLATE : &str = "database_scripts/account_session/get_account_session_by_refresh_token.sql";
    
    let mut context = tera::Context::new();
    context.insert("refresh_token", &refresh_token);
    
    let command = render_query_template(GET_ACCOUNT_SESSION_BY_REFRESH_TOKEN_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_account_session(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn update_account_session_tokens_by_refresh_token(old_refresh_token : &str, auth_token : &str, refresh_token : &str, state : &AppState) -> () {
    const UPDATE_ACCOUNT_SESSION_TOKENS_BY_REFRESH_TOKEN_TEMPLATE : &str = "database_scripts/account_session/update_account_session_tokens_by_refresh_token.sql";
    let now_time = Utc::now();
    
    let mut context = tera::Context::new();
    context.insert("old_refresh_token", &old_refresh_token);
    context.insert("auth_token", &auth_token);
    context.insert("refresh_token", &refresh_token);
    context.insert("now", &now_time.to_rfc3339());

    execute_script_template_wo_return(UPDATE_ACCOUNT_SESSION_TOKENS_BY_REFRESH_TOKEN_TEMPLATE, &context, &state).await;
}

pub async fn update_account_session_last_usage_date_by_id(id : &str, state : &AppState) -> () {
    const UPDATE_ACCOUNT_SESSION_LAST_USAGE_DATE_BY_ID_TEMPLATE : &str = "database_scripts/account_session/update_account_session_last_usage_date_by_id.sql";
    let now_time = Utc::now();
    
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("now", &now_time.to_rfc3339());

    execute_script_template_wo_return(UPDATE_ACCOUNT_SESSION_LAST_USAGE_DATE_BY_ID_TEMPLATE, &context, &state).await;
}

pub async fn update_account_session_last_usage_date_by_token(token : &str, state : &AppState) -> () {
    const UPDATE_ACCOUNT_SESSION_LAST_USAGE_DATE_BY_TOKEN_TEMPLATE : &str = "database_scripts/account_session/update_account_session_last_usage_date_by_token.sql";
    let now_time = Utc::now();
    
    let mut context = tera::Context::new();
    context.insert("token", &token);
    context.insert("now", &now_time.to_rfc3339());

    execute_script_template_wo_return(UPDATE_ACCOUNT_SESSION_LAST_USAGE_DATE_BY_TOKEN_TEMPLATE, &context, &state).await;
}

pub async fn delete_account_session_by_id(id : &str, state : &AppState) -> () {
    const DELETE_ACCOUNT_SESSION_BY_ID_TEMPLATE : &str = "database_scripts/account_session/delete_account_session_by_id.sql";
    
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    execute_script_template_wo_return(DELETE_ACCOUNT_SESSION_BY_ID_TEMPLATE, &context, state).await;
}

pub async fn delete_account_session_by_account_id(account_id : &str, state : &AppState) -> () {
    const DELETE_ACCOUNT_SESSION_BY_ACCOUNT_ID_TEMPLATE : &str = "database_scripts/account_session/delete_account_session_by_account_id.sql";
    
    let mut context = tera::Context::new();
    context.insert("account_id", &account_id);
    
    execute_script_template_wo_return(DELETE_ACCOUNT_SESSION_BY_ACCOUNT_ID_TEMPLATE, &context, state).await;
}

pub async fn delete_account_sessions_with_expiried_refresh_tokens(state : &AppState) -> () {
    const DELETE_ACCOUNT_SESSIONS_WITH_EXPIRIED_REFRESH_TOKENS_TEMPLATE : &str = "database_scripts/account_session/delete_account_sessions_with_expiried_refresh_tokens.sql";
    let now_time = Utc::now();

    let mut context = tera::Context::new();
    context.insert("lifetime", &state.config.lock().await.auth.check_session_status_freq);
    context.insert("now", &now_time.to_rfc3339());
    
    execute_script_template_wo_return(DELETE_ACCOUNT_SESSIONS_WITH_EXPIRIED_REFRESH_TOKENS_TEMPLATE, &context, &state).await;
}

