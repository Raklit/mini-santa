use sqlx::{sqlite::SqliteRow, Row};

use crate::{core::{functions::{command_result_exists, get_one_item_from_command, render_query_template}, services::{IDbService, SQLiteDbService}}, santa::data_model::{implementations::Member, traits::IMember}, AppState};

pub fn row_to_member(row : &SqliteRow) -> Member {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let room_id : &str = row.get("room_id");
    let pool_id : &str = row.get("pool_id");
    let wishlist : &str = row.get("wishlist");
    return Member::new(id, account_id, room_id, pool_id, wishlist);
}

pub async fn get_member_by_id(id : &str, state : &AppState) -> Option<impl IMember> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("members", "id", id, row_to_member).await;
}

pub async fn get_members_by_account_id(account_id : &str, state : &AppState) -> Option<Vec<impl IMember>> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_many_by_prop("members", "account_id", vec![account_id], row_to_member).await;
}

pub async fn get_members_by_room_id(room_id : &str, state : &AppState) -> Option<Vec<impl IMember>> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_many_by_prop("members", "room_id", vec![room_id], row_to_member).await;
}

pub async fn get_members_by_pool_id(pool_id : &str, state : &AppState) -> Option<Vec<impl IMember>> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_many_by_prop("members", "pool_id", vec![pool_id], row_to_member).await;
}

pub async fn get_member_by_pool_and_account_ids(pool_id : &str, account_id : &str, state : &AppState) -> Option<impl IMember> {
    const GET_MEMBER_BY_POOL_AND_ACCOUNT_IDS_TEMPLATE : &str = "database_scripts/member/get_member_by_pool_and_account_ids.sql";
    let mut context = tera::Context::new();
    context.insert("pool_id", &pool_id);
    context.insert("account_id", &account_id);
    
    let command = render_query_template(GET_MEMBER_BY_POOL_AND_ACCOUNT_IDS_TEMPLATE, &context, &state).await;
    return get_one_item_from_command(command.as_str(), state, row_to_member).await;
}

pub async fn is_member_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("members", "id", id).await;
}

pub async fn is_member_already_exists_by_pool_and_account_ids(pool_id : &str, account_id : &str, state : &AppState) -> bool {
    const EXISTS_MEMBER_BY_POOL_AND_ACCOUNT_IDS_TEMPLATE : &str = "database_scripts/member/exists_member_by_pool_and_account_ids.sql";
    let mut context = tera::Context::new();
    context.insert("pool_id", &pool_id);
    context.insert("account_id", &account_id);
    let command = render_query_template(EXISTS_MEMBER_BY_POOL_AND_ACCOUNT_IDS_TEMPLATE, &context, &state).await;
    return command_result_exists(command.as_str(), state).await;
}

pub async fn create_member(id : &str, account_id : &str, room_id : &str, pool_id : &str, wishlist : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let _ = db_service.insert("members", vec!["id", "account_id", "room_id", "pool_id", "wishlist"], vec![vec![id, account_id, room_id, pool_id, wishlist]]).await;
}

pub async fn set_wishlist_by_id(id : &str, wishlist : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    db_service.update("members", "id", id, vec!["wishlist"], vec![wishlist]).await;
}

pub async fn set_member_room_id(id : &str, room_id : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    db_service.update("members", "id", id, vec!["room_id"], vec![room_id]).await;
}

pub async fn delete_member_by_id(id : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    db_service.delete_one_by_prop("members", "id", id).await;
}