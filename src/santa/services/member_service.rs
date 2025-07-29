use sqlx::{sqlite::SqliteRow, Row};

use crate::{core::functions::{command_result_exists, execute_script_template_wo_return, get_many_items_from_command, get_one_item_from_command, render_query_template}, santa::data_model::{implementations::Member, traits::IMember}, AppState};

fn row_to_member(row : &SqliteRow) -> Member {
    let id : &str = row.get("id");
    let account_id : &str = row.get("account_id");
    let room_id : &str = row.get("room_id");
    let pool_id : &str = row.get("pool_id");
    let wishlist : &str = row.get("wishlist");
    return Member::new(id, account_id, room_id, pool_id, wishlist);
}

pub async fn get_member_by_id(id : &str, state : &AppState) -> Option<impl IMember> {
    const GET_MEMBER_BY_ID_TEMPLATE : &str = "database_scripts/member/get_member_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(GET_MEMBER_BY_ID_TEMPLATE, &context, &state).await;
    return get_one_item_from_command(command.as_str(), state, row_to_member).await;
}

pub async fn get_members_by_account_id(account_id : &str, state : &AppState) -> Option<Vec<impl IMember>> {
    const GET_MEMBERS_BY_ACCOUNT_ID_TEMPLATE : &str = "database_scripts/member/get_members_by_account_id.sql";
    let mut context = tera::Context::new();
    context.insert("account_id", &account_id);
    
    let command = render_query_template(GET_MEMBERS_BY_ACCOUNT_ID_TEMPLATE, &context, &state).await;
    return get_many_items_from_command(command.as_str(), state, row_to_member).await;
}

pub async fn get_members_by_room_id(room_id : &str, state : &AppState) -> Option<Vec<impl IMember>> {
    const GET_MEMBERS_BY_ROOM_ID_TEMPLATE : &str = "database_scripts/member/get_members_by_room_id.sql";
    let mut context = tera::Context::new();
    context.insert("room_id", &room_id);
    
    let command = render_query_template(GET_MEMBERS_BY_ROOM_ID_TEMPLATE, &context, &state).await;
    return get_many_items_from_command(command.as_str(), state, row_to_member).await;
}

pub async fn get_members_by_pool_id(pool_id : &str, state : &AppState) -> Option<Vec<impl IMember>> {
    const GET_MEMBERS_BY_ACCOUNT_ID_TEMPLATE : &str = "database_scripts/member/get_members_by_pool_id.sql";
    let mut context = tera::Context::new();
    context.insert("pool_id", &pool_id);
    
    let command = render_query_template(GET_MEMBERS_BY_ACCOUNT_ID_TEMPLATE, &context, &state).await;
    return get_many_items_from_command(command.as_str(), state, row_to_member).await;
}

pub async fn is_member_already_exists_by_id(id : &str, state : &AppState) -> bool {
    const EXISTS_MEMBER_BY_ID_TEMPLATE : &str = "database_scripts/member/exists_member_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    let command = render_query_template(EXISTS_MEMBER_BY_ID_TEMPLATE, &context, &state).await;
    return command_result_exists(command.as_str(), state).await;
}

pub async fn create_member(id : &str, account_id : &str, room_id : &str, wishlist : &str, state : &AppState) -> () {
    let mut context = tera::Context::new();
     context.insert("id", id);
     context.insert("account_id", account_id);
     context.insert("room_id", room_id);
     context.insert("wishlist", wishlist);
 
     const CREATE_MEMBER_TEMPLATE : &str = "database_scripts/member/create_member.sql";
     execute_script_template_wo_return(CREATE_MEMBER_TEMPLATE, &context, &state).await;
}

pub async fn set_wishlist_by_id(id : &str, wishlist : &str, state : &AppState) -> () {
    let mut context = tera::Context::new();
     context.insert("id", id);
     context.insert("wishlist", wishlist);
 
     const SET_WISHLIST_TEMPLATE : &str = "database_scripts/member/set_member_wishlist.sql";
     execute_script_template_wo_return(SET_WISHLIST_TEMPLATE, &context, &state).await;
}

pub async fn delete_member_by_id(id : &str, state : &AppState) -> () {
    let mut context = tera::Context::new();
     context.insert("id", id);
 
     const DELETE_MEMBER_TEMPLATE : &str = "database_scripts/member/delete_member_by_id.sql";
     execute_script_template_wo_return(DELETE_MEMBER_TEMPLATE, &context, &state).await;
}