use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Executor};

use crate::core::data_model::implementations::AuthCode;
use crate::core::data_model::traits::IAuthCode;
use crate::core::functions::{execute_script_template_wo_return, render_query_template};
use crate::AppState;

fn row_to_auth_code(row : &SqliteRow) -> AuthCode {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let code : &str = row.get("code");
    let creation_date_str : &str = row.get("creation_date");
    let creation_date : DateTime<Utc> = DateTime::from_str(creation_date_str).unwrap();
    return AuthCode::new(id, account_id, code, creation_date);
}

pub async fn is_auth_code_already_exists_by_id(id : &str, state : &AppState) -> bool {
    const EXISTS_AUTH_CODE_BY_ID_TEMPLATE : &str = "database_scripts/auth_code/exists_auth_code_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    let command = render_query_template(EXISTS_AUTH_CODE_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn is_auth_code_already_exists_by_code(code : &str, state : &AppState) -> bool {
    const EXISTS_AUTH_CODE_BY_CODE_TEMPLATE : &str = "database_scripts/auth_code/exists_auth_code_by_code.sql";
    let mut context = tera::Context::new();
    context.insert("code", &code);
    let command = render_query_template(EXISTS_AUTH_CODE_BY_CODE_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn create_auth_code(id : &str, account_id : &str, code : &str, state : &AppState) -> () {
    let creation_date = Utc::now().to_rfc3339();
 
     let mut context = tera::Context::new();
     context.insert("id", &id);
     context.insert("account_id", &account_id);
     context.insert("code", &code);
     context.insert("creation_date", &creation_date);
 
    const CREATE_AUTH_CODE : &str = "database_scripts/auth_code/create_auth_code.sql";
    execute_script_template_wo_return(CREATE_AUTH_CODE, &context, &state).await;
 }

pub async fn get_auth_code_by_id(id : &str, state : &AppState) -> Option<impl IAuthCode> {
    const GET_AUTH_CODE_BY_ID_TEMPLATE : &str = "database_scripts/auth_code/get_auth_code_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(GET_AUTH_CODE_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_auth_code(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn get_auth_code_by_code(code : &str, state : &AppState) -> Option<impl IAuthCode> {
    const GET_AUTH_CODE_BY_CODE_TEMPLATE : &str = "database_scripts/auth_code/get_auth_code_by_code.sql";
    let mut context = tera::Context::new();
    context.insert("code", &code);
    
    let command = render_query_template(GET_AUTH_CODE_BY_CODE_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_auth_code(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn delete_auth_code_by_id(id : &str, state : &AppState) -> () {
    const DELETE_AUTH_CODE_BY_ID_TEMPLATE : &str = "database_scripts/auth_code/delete_auth_code_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    execute_script_template_wo_return(DELETE_AUTH_CODE_BY_ID_TEMPLATE, &context, &state).await;
}

pub async fn delete_auth_code_by_code(code : &str, state : &AppState) -> () {
    const DELETE_AUTH_CODE_BY_CODE_TEMPLATE : &str = "database_scripts/auth_code/delete_auth_code_by_code.sql";
    let mut context = tera::Context::new();
    context.insert("code", &code);
    execute_script_template_wo_return(DELETE_AUTH_CODE_BY_CODE_TEMPLATE, &context, &state).await;
}

pub async fn delete_expiried_auth_codes(state : &AppState) -> () {
    const DELETE_EXPIRIED_AUTH_CODES_TEMPLATE : &str = "database_scripts/auth_code/delete_expiried_auth_codes.sql";
    let now_time = Utc::now();

    let mut context = tera::Context::new();
    context.insert("lifetime", &state.config.lock().await.auth.check_auth_code_status_freq);
    context.insert("now", &now_time.to_rfc3339());
    
    execute_script_template_wo_return(DELETE_EXPIRIED_AUTH_CODES_TEMPLATE, &context, &state).await;
}