use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Executor};

use crate::core::data_model::implementations::AuthCode;
use crate::core::data_model::traits::IAuthCode;
use crate::core::functions::{execute_script_template_wo_return, render_query_template};
use crate::core::services::{db_service, IDbService, SQLiteDbService};
use crate::AppState;

pub fn row_to_auth_code(row : &SqliteRow) -> AuthCode {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let code : &str = row.get("code");
    let creation_date_str : &str = row.get("creation_date");
    let creation_date : DateTime<Utc> = DateTime::from_str(creation_date_str).unwrap();
    return AuthCode::new(id, account_id, code, creation_date);
}

pub async fn is_auth_code_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("auth_codes", "id", id).await;
}

pub async fn is_auth_code_already_exists_by_code(code : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("auth_codes", "code", code).await;
}

pub async fn create_auth_code(id : &str, account_id : &str, code : &str, state : &AppState) -> () {
    let creation_date = Utc::now().to_rfc3339();
    
    let db_service = SQLiteDbService::new(state);
    let props = vec!["id", "account_id", "code", "creation_date"];
    let values = vec![vec![id, account_id, code, creation_date.as_str()]];
    let _ = db_service.insert("auth_codes", props, values).await;
 }

pub async fn get_auth_code_by_id(id : &str, state : &AppState) -> Option<impl IAuthCode> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("auth_codes", "id", id, row_to_auth_code).await;
}

pub async fn get_auth_code_by_code(code : &str, state : &AppState) -> Option<impl IAuthCode> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("auth_codes", "code", code, row_to_auth_code).await;
}

pub async fn delete_auth_code_by_id(id : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let _ = db_service.delete_one_by_prop("auth_codes", "id", id).await;
}

pub async fn delete_auth_code_by_code(code : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let _ = db_service.delete_one_by_prop("auth_codes", "code", code).await;
}

pub async fn delete_expiried_auth_codes(state : &AppState) -> () {
    const DELETE_EXPIRIED_AUTH_CODES_TEMPLATE : &str = "database_scripts/auth_code/delete_expiried_auth_codes.sql";
    let now_time = Utc::now();

    let mut context = tera::Context::new();
    context.insert("lifetime", &state.config.lock().await.auth.check_auth_code_status_freq);
    context.insert("now", &now_time.to_rfc3339());
    
    execute_script_template_wo_return(DELETE_EXPIRIED_AUTH_CODES_TEMPLATE, &context, &state).await;
}