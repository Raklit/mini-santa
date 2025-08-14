use std::str::FromStr;

use chrono::prelude::*;
use sqlx::{Row, Executor};
use sqlx::sqlite::SqliteRow;

use crate::core::data_model::implementations::AccountSession;
use crate::core::data_model::traits::IAccountSession;
use crate::core::functions::{execute_script_template_wo_return, render_query_template};
use crate::core::services::{db_service, IDbService, SQLiteDbService};
use crate::AppState;

pub fn row_to_account_session(row : &SqliteRow) -> AccountSession {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let access_token : &str = row.get("access_token");
    let refresh_token : &str = row.get("refresh_token");

    let start_date_str : &str = row.get("start_date");
    let access_token_creation_date_str : &str = row.get("access_token_creation_date");
    let refresh_token_creation_date_str : &str = row.get("refresh_token_creation_date");
    let last_usage_date_str : &str = row.get("last_usage_date");

    let start_date : DateTime<Utc> = DateTime::from_str(start_date_str).unwrap();
    let access_token_creation_date : DateTime<Utc> = DateTime::from_str(access_token_creation_date_str).unwrap();
    let refresh_token_creation_date : DateTime<Utc> = DateTime::from_str(refresh_token_creation_date_str).unwrap();
    let last_usage_date : DateTime<Utc> = DateTime::from_str(last_usage_date_str).unwrap();

    return AccountSession::new(id, account_id, access_token, refresh_token, start_date, access_token_creation_date, refresh_token_creation_date, last_usage_date);
}

pub async fn is_account_session_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("account_sessions", "id", id).await;
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

pub async fn create_account_session(id : &str, account_id : &str, access_token : &str, refresh_token : &str, state : &AppState) -> () {
    let creation_date = Utc::now().to_rfc3339();
    let creation_date_str = creation_date.as_str();

    let db_service = SQLiteDbService::new(state);
    let props = vec!["id", "account_id", "access_token", "refresh_token", "start_date", "access_token_creation_date", "refresh_token_creation_date", "last_usage_date"];
    let values = vec![vec![id, account_id, access_token, refresh_token, creation_date_str, creation_date_str, creation_date_str, creation_date_str]];
    let _ = db_service.insert("account_sessions", props, values).await;
 }

pub async fn get_account_session_by_ids(ids : Vec<&str>, state : &AppState) -> Option<Vec<impl IAccountSession>> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_many_by_prop("account_sessions", "id", ids, row_to_account_session).await;
}

pub async fn get_account_session_by_id(id : &str, state : &AppState) -> Option<impl IAccountSession> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("account_sessions", "id", id, row_to_account_session).await;
}

pub async fn get_account_session_by_access_token(access_token : &str, state : &AppState) -> Option<impl IAccountSession> {
   let db_service = SQLiteDbService::new(state);
   return db_service.get_one_by_prop("account_sessions", "access_token", access_token, row_to_account_session).await;
}

pub async fn get_account_session_by_refresh_token(refresh_token : &str, state : &AppState) -> Option<impl IAccountSession> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("account_sessions", "refresh_token", refresh_token, row_to_account_session).await;
}

pub async fn update_account_session_tokens_by_refresh_token(old_refresh_token : &str, access_token : &str, refresh_token : &str, state : &AppState) -> () {
    const UPDATE_ACCOUNT_SESSION_TOKENS_BY_REFRESH_TOKEN_TEMPLATE : &str = "database_scripts/account_session/update_account_session_tokens_by_refresh_token.sql";
    let now_time = Utc::now();
    
    let mut context = tera::Context::new();
    context.insert("old_refresh_token", &old_refresh_token);
    context.insert("access_token", &access_token);
    context.insert("refresh_token", &refresh_token);
    context.insert("now", &now_time.to_rfc3339());

    execute_script_template_wo_return(UPDATE_ACCOUNT_SESSION_TOKENS_BY_REFRESH_TOKEN_TEMPLATE, &context, &state).await;
}

pub async fn update_account_session_last_usage_date_by_id(id : &str, state : &AppState) -> () {
    let now_time = Utc::now().to_rfc3339();
    let db_service = SQLiteDbService::new(state);
    let props = vec!["last_usage_date"];
    let values = vec![now_time.as_str()];
    return db_service.update("account_sessions", "id", id, props, values).await;
}

pub async fn update_account_session_last_usage_date_by_token(token : &str, state : &AppState) -> () {
    const UPDATE_ACCOUNT_SESSION_LAST_USAGE_DATE_BY_TOKEN_TEMPLATE : &str = "database_scripts/account_session/update_account_session_last_usage_date_by_token.sql";
    let now_time = Utc::now();
    
    let mut context = tera::Context::new();
    context.insert("token", &token);
    context.insert("now", &now_time.to_rfc3339());

    execute_script_template_wo_return(UPDATE_ACCOUNT_SESSION_LAST_USAGE_DATE_BY_TOKEN_TEMPLATE, &context, &state).await;
}

pub async fn delete_account_session_by_ids(ids : Vec<&str>, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let _ = db_service.delete_many_by_prop("account_sessions", "id", ids).await;
}

pub async fn delete_account_session_by_id(id : &str, state : &AppState) -> () {
    delete_account_session_by_ids(vec![id], state).await;
}

pub async fn delete_account_sessions_by_account_id(account_id : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let _ = db_service.delete_many_by_prop("account_sessions", "account_id", vec![account_id]).await;
}

pub async fn delete_account_sessions_with_expiried_refresh_tokens(state : &AppState) -> () {
    const DELETE_ACCOUNT_SESSIONS_WITH_EXPIRIED_REFRESH_TOKENS_TEMPLATE : &str = "database_scripts/account_session/delete_account_sessions_with_expiried_refresh_tokens.sql";
    let now_time = Utc::now();

    let mut context = tera::Context::new();
    context.insert("lifetime", &state.config.lock().await.auth.check_session_status_freq);
    context.insert("now", &now_time.to_rfc3339());
    
    execute_script_template_wo_return(DELETE_ACCOUNT_SESSIONS_WITH_EXPIRIED_REFRESH_TOKENS_TEMPLATE, &context, &state).await;
}

