use sqlx::{Row, Executor};
use sqlx::sqlite::SqliteRow;

use crate::core::data_model::implementations::RecoveryUserInfo;
use crate::core::data_model::traits::IRecoveryUserInfo;
use crate::core::functions::{execute_script_template_wo_return, render_query_template};
use crate::AppState;

fn row_to_recovery_user_info(row : &SqliteRow) -> RecoveryUserInfo {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let email : &str = row.get("email");
    let phone : &str = row.get("phone");

    return RecoveryUserInfo::new(id, account_id, email, phone);
}

pub async fn is_recovery_user_info_already_exists_by_id(id : &str, state : &AppState) -> bool {
    const EXISTS_RECOVERY_USER_INFO_BY_ID_TEMPLATE : &str = "database_scripts/recovery_user_info/exists_recovery_user_info_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", id);
    let command = render_query_template(EXISTS_RECOVERY_USER_INFO_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    
    return val == 1;
}

pub async fn is_recovery_user_info_already_exists_by_email(email : &str, state : &AppState) -> bool {
    const EXISTS_RECOVERY_USER_INFO_BY_EMAIL_TEMPLATE : &str = "database_scripts/recovery_user_info/exists_recovery_user_info_by_email.sql";
    let mut context = tera::Context::new();
    context.insert("email", email);
    let command = render_query_template(EXISTS_RECOVERY_USER_INFO_BY_EMAIL_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    
    return val == 1;
}

pub async fn create_recovery_user_info(id : &str, account_id : &str, email : &str, phone : &str, state : &AppState) -> () {
 
     let mut context = tera::Context::new();
     context.insert("id", id);
     context.insert("account_id", account_id);
     context.insert("email", email);
     context.insert("phone", phone);

     const CREATE_RECOVERY_USER_INFO_TEMPLATE : &str = "database_scripts/recovery_user_info/create_recovery_user_info.sql";
     execute_script_template_wo_return(CREATE_RECOVERY_USER_INFO_TEMPLATE, &context, &state).await;
 }

pub async fn get_recovery_user_info_by_id(id : &str, state : &AppState) -> Option<impl IRecoveryUserInfo> {
    const GET_RECOVERY_USER_INFO_BY_ID_TEMPLATE : &str = "database_scripts/recovery_user_info/get_recovery_user_info_by_id.sql";
    
    let mut context = tera::Context::new();
    context.insert("id", id);
    
    let command = render_query_template(GET_RECOVERY_USER_INFO_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_recovery_user_info(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn get_recovery_user_info_by_account_id(account_id : &str, state : &AppState) -> Option<impl IRecoveryUserInfo> {
    const GET_RECOVERY_USER_INFO_BY_ACCOUNT_ID_TEMPLATE : &str = "database_scripts/recovery_user_info/get_recovery_user_info_by_account_id.sql";
    
    let mut context = tera::Context::new();
    context.insert("account_id", account_id);
    
    let command = render_query_template(GET_RECOVERY_USER_INFO_BY_ACCOUNT_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_recovery_user_info(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn set_email(account_id : &str, email : &str, state : &AppState) -> () {
    const SET_EMAIL_TEMPLATE : &str = "database_scripts/recovery_user_info/set_email.sql";
    let mut context = tera::Context::new();
    context.insert("account_id", account_id);
    context.insert("email", email);
    execute_script_template_wo_return(SET_EMAIL_TEMPLATE, &context, &state).await;
}

pub async fn set_phone(account_id : &str, phone : &str, state : &AppState) -> () {
    const SET_PHONE_TEMPLATE : &str = "database_scripts/recovery_user_info/set_phone.sql";
    let mut context = tera::Context::new();
    context.insert("account_id", account_id);
    context.insert("phone", phone);
    execute_script_template_wo_return(SET_PHONE_TEMPLATE, &context, &state).await;
}
