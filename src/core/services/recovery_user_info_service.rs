use sqlx::{Row, Executor};
use sqlx::sqlite::SqliteRow;

use crate::core::data_model::implementations::RecoveryUserInfo;
use crate::core::data_model::traits::IRecoveryUserInfo;
use crate::core::functions::{execute_script_template_wo_return, render_query_template};
use crate::core::services::{IDbService, SQLiteDbService};
use crate::AppState;

pub fn row_to_recovery_user_info(row : &SqliteRow) -> RecoveryUserInfo {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let email : &str = row.get("email");
    let phone : &str = row.get("phone");

    return RecoveryUserInfo::new(id, account_id, email, phone);
}

pub async fn is_recovery_user_info_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("recovery_user_infos", "id", id).await;
}

pub async fn is_recovery_user_info_already_exists_by_email(email : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("recovery_user_infos", "email", email).await;
}

pub async fn create_recovery_user_info(id : &str, account_id : &str, email : &str, phone : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["id", "account_id", "email", "phone"];
    let values = vec![vec![id, account_id, email, phone]];
    let _ = db_service.insert("recovery_user_infos", props, values).await;
 }

pub async fn get_recovery_user_info_by_id(id : &str, state : &AppState) -> Option<impl IRecoveryUserInfo> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("recovery_user_infos", "id", id, row_to_recovery_user_info).await;
}

pub async fn get_recovery_user_info_by_account_id(account_id : &str, state : &AppState) -> Option<impl IRecoveryUserInfo> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("recovery_user_infos", "account_id", account_id, row_to_recovery_user_info).await;
}

pub async fn set_email(account_id : &str, email : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["email"];
    let values = vec![email];
    let _ = db_service.update("recovery_user_infos", "account_id", account_id, props, values).await;
}

pub async fn set_phone(account_id : &str, phone : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["phone"];
    let values = vec![phone];
    let _ = db_service.update("recovery_user_infos", "account_id", account_id, props, values).await;
}
