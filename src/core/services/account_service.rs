use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Executor};

use crate::core::data_model::implementations::Account;
use crate::core::data_model::traits::IAccount;
use crate::core::functions::{execute_script_template_wo_return, hash_password, render_query_template};
use crate::core::services::db_service::{self, SQLiteDbService};
use crate::core::services::IDbService;
use crate::AppState;

pub fn row_to_account(row : &SqliteRow) -> Account {
    let id : &str = row.get("id");
    let login : &str = row.get("login");
    let password_hash : &str = row.get("password_hash");
    let password_salt : &str = row.get("password_salt");
    return Account::new(id, login, password_hash, password_salt);
}

pub async fn is_account_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("accounts", "id", id).await;

}

pub async fn is_account_already_exists_by_login(login : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("accounts", "login", login).await;
}

pub async fn create_account(id : &str, login : &str, password : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let [pwd_hash, pwd_salt] = hash_password(&password);
    let props = vec!["id", "login", "password_hash", "password_salt"];
    let values = vec![vec![id, login, pwd_hash.as_str(), pwd_salt.as_str()]];
    let _ = db_service.insert("accounts", props, values).await;
 }

pub async fn get_account_by_id(id : &str, state : &AppState) -> Option<impl IAccount> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("accounts", "id", id, row_to_account).await;
}

pub async fn get_account_by_login(login : &str, state : &AppState) -> Option<impl IAccount> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("accounts", "login", login, row_to_account).await;
}

pub async fn set_account_login(id : &str, login : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["login"];
    let values = vec![login];
    let _ = db_service.update("accounts", "id", id, props, values).await;
}

pub async fn set_account_password(id : &str, password : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let [pwd_hash, pwd_salt] = hash_password(password);
    let props = vec!["password_hash", "password_salt"];
    let values = vec![pwd_hash.as_str(), pwd_salt.as_str()];
    let _ = db_service.update("accounts", "id", id, props, values).await;
}

pub async fn delete_account_by_ids(ids : Vec<&str>, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let _ = db_service.delete_many_by_prop("accounts", "id", ids).await;
}

pub async fn delete_account_by_logins(logins : Vec<&str>, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let _ = db_service.delete_many_by_prop("accounts", "login", logins).await;
}

pub async fn delete_account_by_id(id : &str, state : &AppState) -> () {
    delete_account_by_ids(vec![id], state).await;
}

pub async fn delete_account_by_login(login : &str, state : &AppState) -> () {
    delete_account_by_logins(vec![login], state).await;
}