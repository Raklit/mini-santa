use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Executor};

use crate::core::data_model::implementations::Client;
use crate::core::data_model::traits::IClient;
use crate::core::functions::{execute_script_template_wo_return, hash_password, render_query_template};
use crate::AppState;

fn row_to_client(row : &SqliteRow) -> Client {
    let id : &str = row.get("id");
    let client_name : &str = row.get("client_name");
    let password_hash : &str = row.get("password_hash");
    let password_salt : &str = row.get("password_salt");
    let redirect_uri : &str = row.get("redirect_uri");
    let is_public : bool = row.get("is_public");
    return Client::new(id, client_name, password_hash, password_salt, redirect_uri, is_public);
}

pub async fn is_client_already_exists_by_id(id : &str, state : &AppState) -> bool {
    const EXISTS_CLIENT_BY_ID_TEMPLATE : &str = "database_scripts/client/exists_client_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    let command = render_query_template(EXISTS_CLIENT_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn is_client_already_exists_by_client_name(client_name : &str, state : &AppState) -> bool {
    const EXISTS_CLIENT_BY_CLIENT_NAME_TEMPLATE : &str = "database_scripts/client/exists_client_by_client_name.sql";
    let mut context = tera::Context::new();
    context.insert("client_name", &client_name);
    let command = render_query_template(EXISTS_CLIENT_BY_CLIENT_NAME_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn is_client_already_exists_by_id_or_client_name(id : &str, client_name : &str, state : &AppState) -> bool {
    const EXISTS_CLIENT_BY_ID_OR_CLIENT_NAME_TEMPLATE : &str = "database_scripts/client/exists_client_by_id_or_client_name.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("cliennt_name", &client_name);
    let command = render_query_template(EXISTS_CLIENT_BY_ID_OR_CLIENT_NAME_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn create_client(id : &str, client_name : &str, password : &str, redirect_uri : &str, is_public : bool, state : &AppState) -> () {
    let [pwd_hash, pwd_salt] = hash_password(&password);
 
     let mut context = tera::Context::new();
     context.insert("id", id);
     context.insert("client_name", client_name);
     context.insert("password_hash", pwd_hash.as_str());
     context.insert("password_salt", pwd_salt.as_str());
     context.insert("redirect_uri", redirect_uri);
     context.insert("is_public", &is_public);
 
     const CREATE_CLIENT_TEMPLATE : &str = "database_scripts/client/create_client.sql";
     execute_script_template_wo_return(CREATE_CLIENT_TEMPLATE, &context, &state).await;
 }

pub async fn get_client_by_id(id : &str, state : &AppState) -> Option<impl IClient> {
    const GET_CLIENT_BY_ID_TEMPLATE : &str = "database_scripts/client/get_client_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(GET_CLIENT_BY_ID_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_client(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn get_client_by_client_name(client_name : &str, state : &AppState) -> Option<impl IClient> {
    const GET_CLIENT_BY_CLIENT_NAME_TEMPLATE : &str = "database_scripts/client/get_client_by_client_name.sql";
    let mut context = tera::Context::new();
    context.insert("client_name", &client_name);
    
    let command = render_query_template(GET_CLIENT_BY_CLIENT_NAME_TEMPLATE, &context, &state).await;
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command.as_str()).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        return Some(row_to_client(&result.unwrap()));
    } else {
        return None;
    }
}

pub async fn set_client_name(id : &str, client_name : &str, state : &AppState) -> () {
    const SET_CLIENT_NAME_TEMPLATE : &str = "database_scripts/client/set_client_name.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("client_name", &client_name);
    execute_script_template_wo_return(SET_CLIENT_NAME_TEMPLATE, &context, &state).await;
}

pub async fn set_client_password(id : &str, password : &str, state : &AppState) -> () {
    let hashed_password = hash_password(&password);

    const SET_CLIENT_PASSWORD_TEMPLATE : &str = "database_scripts/client/set_client_password.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("password_hash", hashed_password[0].as_str());
    context.insert("password_salt", hashed_password[1].as_str());
    execute_script_template_wo_return(SET_CLIENT_PASSWORD_TEMPLATE, &context, &state).await;
}

pub async fn set_client_redirect_uri(id : &str, redirect_uri : &str, state : &AppState) -> () {
    const SET_CLIENT_REDIRECT_URI_TEMPLATE : &str = "database_scripts/client/set_client_redirect_uri.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("redirect_uri", &redirect_uri);
    execute_script_template_wo_return(SET_CLIENT_REDIRECT_URI_TEMPLATE, &context, &state).await;
}

pub async fn delete_client_by_id(id : &str, state : &AppState) -> () {
    const DELETE_CLIENT_BY_ID_TEMPLATE : &str = "database_scripts/client/delete_client_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    execute_script_template_wo_return(DELETE_CLIENT_BY_ID_TEMPLATE, &context, &state).await;
}

pub async fn delete_client_by_client_name(client_name : &str, state : &AppState) -> () {
    const DELETE_CLIENT_BY_CLIENT_NAME_TEMPLATE : &str = "database_scripts/client/delete_client_by_client_name.sql";
    let mut context = tera::Context::new();
    context.insert("client_name", &client_name);
    execute_script_template_wo_return(DELETE_CLIENT_BY_CLIENT_NAME_TEMPLATE, &context, &state).await;
}