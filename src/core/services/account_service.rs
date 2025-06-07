use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Executor};

use crate::core::data_model::implementations::Account;
use crate::core::data_model::traits::IAccount;
use crate::core::functions::{execute_script_template_wo_return, hash_password, render_query_template};
use crate::AppState;

fn row_to_account(row : &SqliteRow) -> Account {
    let id : &str = row.get("id");
    let login : &str = row.get("login");
    let password_hash : &str = row.get("password_hash");
    let password_salt : &str = row.get("password_salt");
    return Account {
        id : String::from(id),
        login : String::from(login),
        password_hash : String::from(password_hash),
        password_salt : String::from(password_salt)
    }
}

pub async fn is_account_already_exists_by_id(id : &str, state : &AppState) -> bool {
    const EXISTS_ACCOUNT_BY_ID_TEMPLATE : &str = "database_scripts/account/exists_account_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    let command = render_query_template(EXISTS_ACCOUNT_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn is_account_already_exists_by_login(login : &str, state : &AppState) -> bool {
    const EXISTS_ACCOUNT_BY_LOGIN_TEMPLATE : &str = "database_scripts/account/exists_account_by_login.sql";
    let mut context = tera::Context::new();
    context.insert("login", &login);
    let command = render_query_template(EXISTS_ACCOUNT_BY_LOGIN_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn is_account_already_exists_by_id_or_login(id : &str, login : &str, state : &AppState) -> bool {
    const EXISTS_ACCOUNT_BY_ID_OR_LOGIN_TEMPLATE : &str = "database_scripts/account/exists_account_by_id_or_login.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("login", &login);
    let command = render_query_template(EXISTS_ACCOUNT_BY_ID_OR_LOGIN_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn create_account(id : &str, login : &str, password : &str, state : &AppState) -> () {
    let hashed_password = hash_password(&password);
    let account = Account {
         id : String::from(id),
         login : String::from(login),
         password_hash : hashed_password[0].clone(),
         password_salt : hashed_password[1].clone()
     };
 
     let mut context = tera::Context::new();
     context.insert("id", &account.id.as_str());
     context.insert("login", &account.login.as_str());
     context.insert("password_hash", &account.password_hash.as_str());
     context.insert("password_salt", &account.password_salt.as_str());
 
    const CREATE_ACCOUNT_TEMPLATE : &str = "database_scripts/account/create_account.sql";
    execute_script_template_wo_return(CREATE_ACCOUNT_TEMPLATE, &context, &state).await;
 }

pub async fn get_account_by_id(id : &str, state : &AppState) -> Option<impl IAccount> {
    const GET_ACCOUNT_BY_ID_TEMPLATE : &str = "database_scripts/account/get_account_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(GET_ACCOUNT_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_account(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn get_account_by_login(login : &str, state : &AppState) -> Option<impl IAccount> {
    const GET_ACCOUNT_BY_LOGIN_TEMPLATE : &str = "database_scripts/account/get_account_by_login.sql";
    let mut context = tera::Context::new();
    context.insert("login", &login);
    
    let command = render_query_template(GET_ACCOUNT_BY_LOGIN_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_account(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn set_account_login(id : &str, login : &str, state : &AppState) -> () {
    const SET_ACCOUNT_LOGIN_TEMPLATE : &str = "database_scripts/account/set_account_login.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("login", &login);
    execute_script_template_wo_return(SET_ACCOUNT_LOGIN_TEMPLATE, &context, &state).await;
}

pub async fn set_account_password(id : &str, password : &str, state : &AppState) -> () {
    let hashed_password = hash_password(&password);

    const SET_ACCOUNT_PASSWORD_TEMPLATE : &str = "database_scripts/account/set_account_password.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("password_hash", hashed_password[0].as_str());
    context.insert("password_salt", hashed_password[1].as_str());
    execute_script_template_wo_return(SET_ACCOUNT_PASSWORD_TEMPLATE, &context, &state).await;
}

pub async fn delete_account_by_id(id : &str, state : &AppState) -> () {
    const DELETE_ACCOUNT_BY_ID_TEMPLATE : &str = "database_scripts/account/delete_account_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    execute_script_template_wo_return(DELETE_ACCOUNT_BY_ID_TEMPLATE, &context, &state).await;
}

pub async fn delete_account_by_login(login : &str, state : &AppState) -> () {
    const DELETE_ACCOUNT_BY_LOGIN_TEMPLATE : &str = "database_scripts/account/delete_account_by_login.sql";
    let mut context = tera::Context::new();
    context.insert("login", &login);
    execute_script_template_wo_return(DELETE_ACCOUNT_BY_LOGIN_TEMPLATE, &context, &state).await;
}