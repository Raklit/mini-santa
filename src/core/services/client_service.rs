use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Executor};

use crate::core::data_model::implementations::Client;
use crate::core::data_model::traits::IClient;
use crate::core::functions::{execute_script_template_wo_return, get_one_item_from_command, hash_password, render_query_template};
use crate::core::services::{IDbService, SQLiteDbService};
use crate::AppState;

pub fn row_to_client(row : &SqliteRow) -> Client {
    let id : &str = row.get("id");
    let client_name : &str = row.get("client_name");
    let password_hash : &str = row.get("password_hash");
    let password_salt : &str = row.get("password_salt");
    let redirect_uri : &str = row.get("redirect_uri");
    let no_pwd_str : &str = row.get("no_pwd");
    let no_pwd = no_pwd_str.to_lowercase() == "true";
    return Client::new(id, client_name, password_hash, password_salt, redirect_uri, no_pwd);
}

pub async fn is_client_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("clients", "id", id).await;
}

pub async fn is_client_already_exists_by_client_name(client_name : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("clients", "client_name", client_name).await;
}

pub async fn create_client(id : &str, client_name : &str, password : &str, redirect_uri : &str, no_pwd : bool, state : &AppState) -> () {
    let [pwd_hash, pwd_salt] = hash_password(&password);
    
    let no_pwd_str : &str;
    if no_pwd {
        no_pwd_str = "true";
    } else {
        no_pwd_str = "false";
    }

    let db_service = SQLiteDbService::new(state);
    let props = vec!["id", "client_name", "password_hash", "password_salt", "redirect_uri", "no_pwd"];
    let values = vec![vec![id, client_name, pwd_hash.as_str(), pwd_salt.as_str(), redirect_uri, no_pwd_str]];

    let _ = db_service.insert("clients", props, values).await;
 }

pub async fn get_client_by_id(id : &str, state : &AppState) -> Option<impl IClient> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("clients", "id", id, row_to_client).await;
}

pub async fn get_client_by_client_name(client_name : &str, state : &AppState) -> Option<impl IClient> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("clients", "client_name", client_name, row_to_client).await;
}

pub async fn set_client_name(id : &str, client_name : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["client_name"];
    let values = vec![client_name];
    return db_service.update("clients", "id", id, props, values).await;
}

pub async fn set_client_password(id : &str, password : &str, state : &AppState) -> () {
    let [pwd_hash, pwd_salt] = hash_password(&password);

    let db_service = SQLiteDbService::new(state);
    let props = vec!["password_hash", "password_salt"];
    let values = vec![pwd_hash.as_str(), pwd_salt.as_str()];
    return db_service.update("clients", "id", id, props, values).await;
}

pub async fn set_client_redirect_uri(id : &str, redirect_uri : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["redirect_uri"];
    let values = vec![redirect_uri];
    return db_service.update("clients", "id", id, props, values).await;
}

pub async fn delete_client_by_id(id : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let _ = db_service.delete_one_by_prop("clients", "id", id).await;
}

pub async fn delete_client_by_client_name(client_name : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let _ = db_service.delete_one_by_prop("clients", "client_name", client_name).await;
}