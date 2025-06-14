use sqlx::{Row, Executor};
use sqlx::sqlite::SqliteRow;

use crate::core::data_model::traits::IPublicUserInfo;
use crate::core::functions::{execute_script_template_wo_return, render_query_template};
use crate::{core::data_model::implementations::PublicUserInfo, AppState};

fn row_to_public_user_info(row : &SqliteRow) -> PublicUserInfo {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let nickname : &str = row.get("nickname");
    let info : &str = row.get("info");

    return PublicUserInfo::new(id, account_id, nickname, info);
}

pub async fn is_public_user_info_already_exists_by_id(id : &str, state : &AppState) -> bool {
    const EXISTS_PUBLIC_USER_INFO_BY_ID_TEMPLATE : &str = "database_scripts/public_user_info/exists_public_user_info_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", id);
    let command = render_query_template(EXISTS_PUBLIC_USER_INFO_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    
    return val == 1;
}

pub async fn is_public_user_info_already_exists_by_nickname(nickname : &str, state : &AppState) -> bool {
    const EXISTS_PUBLIC_USER_INFO_BY_NICKNAME_TEMPLATE : &str = "database_scripts/public_user_info/exists_public_user_info_by_nickname.sql";
    let mut context = tera::Context::new();
    context.insert("nickname", nickname);
    let command = render_query_template(EXISTS_PUBLIC_USER_INFO_BY_NICKNAME_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    
    return val == 1;
}

pub async fn create_public_user_info(id : &str, account_id : &str, nickname : &str, info : &str, state : &AppState) -> () {
 
     let mut context = tera::Context::new();
     context.insert("id", id);
     context.insert("account_id", account_id);
     context.insert("nickname", nickname);
     context.insert("info", info);

     const CREATE_PUBLIC_USER_INFO_TEMPLATE : &str = "database_scripts/public_user_info/create_public_user_info.sql";
     execute_script_template_wo_return(CREATE_PUBLIC_USER_INFO_TEMPLATE, &context, &state).await;
 }

pub async fn get_public_user_info_by_id(id : &str, state : &AppState) -> Option<impl IPublicUserInfo> {
    const GET_PUBLIC_USER_INFO_BY_ID_TEMPLATE : &str = "database_scripts/public_user_info/get_public_user_info_by_id.sql";
    
    let mut context = tera::Context::new();
    context.insert("id", id);
    
    let command = render_query_template(GET_PUBLIC_USER_INFO_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_public_user_info(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn get_public_user_info_by_account_id(account_id : &str, state : &AppState) -> Option<impl IPublicUserInfo> {
    const GET_PUBLIC_USER_INFO_BY_ACCOUNT_ID_TEMPLATE : &str = "database_scripts/public_user_info/get_public_user_info_by_account_id.sql";
    
    let mut context = tera::Context::new();
    context.insert("account_id", account_id);
    
    let command = render_query_template(GET_PUBLIC_USER_INFO_BY_ACCOUNT_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_public_user_info(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn get_public_user_info_by_nickname(nickname : &str, state : &AppState) -> Option<impl IPublicUserInfo> {
    const GET_PUBLIC_USER_INFO_BY_NICKNAME_TEMPLATE : &str = "database_scripts/public_user_info/get_public_user_info_by_nickname.sql";
    
    let mut context = tera::Context::new();
    context.insert("nickname", nickname);
    
    let command = render_query_template(GET_PUBLIC_USER_INFO_BY_NICKNAME_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_public_user_info(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn set_nickname(account_id : &str, nickname : &str, state : &AppState) -> () {
    const SET_NICKNAME_TEMPLATE : &str = "database_scripts/public_user_info/set_nickname.sql";
    let mut context = tera::Context::new();
    context.insert("account_id", account_id);
    context.insert("nickname", nickname);
    execute_script_template_wo_return(SET_NICKNAME_TEMPLATE, &context, &state).await;
}

pub async fn set_info(account_id : &str, info : &str, state : &AppState) -> () {
    const SET_INFO_TEMPLATE : &str = "database_scripts/public_user_info/set_info.sql";
    let mut context = tera::Context::new();
    context.insert("account_id", account_id);
    context.insert("info", info);
    execute_script_template_wo_return(SET_INFO_TEMPLATE, &context, &state).await;
}
