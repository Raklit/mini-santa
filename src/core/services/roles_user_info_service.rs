use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::core::data_model::implementations::RolesUserInfo;
use crate::core::data_model::traits::IRolesUserInfo;
use crate::core::services::{IDbService, SQLiteDbService};
use crate::AppState;

pub fn row_to_roles_user_info(row : &SqliteRow) -> RolesUserInfo {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let role_id : &str = row.get("role_id");
    let params : &str = row.get("params");

    return RolesUserInfo::new(id, account_id, role_id, params);
}

pub async fn is_roles_user_info_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("roles_user_infos", "id", id).await;
}

pub async fn is_roles_user_info_already_exists_by_account_id(account_id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("roles_user_infos", "account_id", account_id).await;
}

pub async fn create_roles_user_info(id : &str, account_id : &str, role_id : &str, params : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["id", "account_id", "role_id", "params"];
    let values = vec![vec![id, account_id, role_id, params]];
    let _ = db_service.insert("roles_user_infos", props, values).await;
 }

pub async fn get_roles_user_info_by_id(id : &str, state : &AppState) -> Option<impl IRolesUserInfo> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("roles_user_infos", "id", id, row_to_roles_user_info).await;
}

pub async fn get_roles_user_info_by_account_id(account_id : &str, state : &AppState) -> Option<impl IRolesUserInfo> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("roles_user_infos", "account_id", account_id, row_to_roles_user_info).await;
}
