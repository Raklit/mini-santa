use sqlx::{Row, Executor};
use sqlx::sqlite::SqliteRow;

use crate::core::data_model::traits::IPublicUserInfo;
use crate::core::functions::{execute_script_template_wo_return, render_query_template};
use crate::core::services::{IDbService, SQLiteDbService};
use crate::{core::data_model::implementations::PublicUserInfo, AppState};

pub fn row_to_public_user_info(row : &SqliteRow) -> PublicUserInfo {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let nickname : &str = row.get("nickname");
    let info : &str = row.get("info");

    return PublicUserInfo::new(id, account_id, nickname, info);
}

pub async fn is_public_user_info_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("public_user_infos", "id", id).await;
}

pub async fn is_public_user_info_already_exists_by_nickname(nickname : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("public_user_infos", "nickname", nickname).await;
}

pub async fn create_public_user_info(id : &str, account_id : &str, nickname : &str, info : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["id", "account_id", "nickname", "info"];
    let values = vec![vec![id, account_id, nickname, info]];
    let _ = db_service.insert("public_user_infos", props, values).await;
 }

pub async fn get_public_user_info_by_id(id : &str, state : &AppState) -> Option<impl IPublicUserInfo> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("public_user_infos", "id", id, row_to_public_user_info).await;
}

pub async fn get_public_user_info_by_account_id(account_id : &str, state : &AppState) -> Option<impl IPublicUserInfo> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("public_user_infos", "account_id", account_id, row_to_public_user_info).await;
}

pub async fn get_public_user_info_by_nickname(nickname : &str, state : &AppState) -> Option<impl IPublicUserInfo> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("public_user_infos", "nickname", nickname, row_to_public_user_info).await;
}

pub async fn set_nickname(account_id : &str, nickname : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["nickname"];
    let values = vec![nickname];
    let _ = db_service.update("public_user_infos", "account_id", account_id, props, values).await;
}

pub async fn set_info(account_id : &str, info : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["info"];
    let values = vec![info];
    let _ = db_service.update("public_user_infos", "account_id", account_id, props, values).await;
}
