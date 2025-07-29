use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqliteRow, Executor, Row};

use crate::{core::functions::{command_result_exists, execute_script_template_wo_return, get_many_items_from_command, get_one_item_from_command, render_query_template}, santa::data_model::{enums::RoomState, implementations::{Message, Room}, traits::{IMessage, IRoom}}, AppState};

fn row_to_message(row : &SqliteRow) -> Message {
    let id : &str = row.get("id");
    let text_content : &str = row.get("text_content");
    let account_id : &str = row.get("account_id");
    let room_id : &str = row.get("room_id");
    let pool_id : &str = row.get("pool_id");
    let creation_date_str : &str = row.get("creation_date");
    let creation_date : DateTime<Utc> = DateTime::from_str(creation_date_str).unwrap();
    return Message::new(id, text_content, account_id, room_id, pool_id, creation_date);
}

pub async fn get_message_by_id(id : &str, state : &AppState) -> Option<impl IMessage> {
    const GET_ROOM_BY_ID_TEMPLATE : &str = "database_scripts/room/get_room_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(GET_ROOM_BY_ID_TEMPLATE, &context, &state).await;
    return get_one_item_from_command(command.as_str(), state, row_to_message).await;
}

pub async fn get_messages_by_account_id(account_id : &str, state : &AppState) -> Option<Vec<impl IMessage>> {
    const GET_MESSAGES_BY_ACCOUNT_ID_TEMPLATE : &str = "database_scripts/messages/get_messages_by_account_id.sql";
    let mut context = tera::Context::new();
    context.insert("account_id", &account_id);
    
    let command = render_query_template(GET_MESSAGES_BY_ACCOUNT_ID_TEMPLATE, &context, &state).await;
    return get_many_items_from_command(command.as_str(), state, row_to_message).await;
}

pub async fn is_message_already_exists_by_id(id : &str, state : &AppState) -> bool {
    const EXISTS_MESSAGE_BY_ID_TEMPLATE : &str = "database_scripts/message/exists_message_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    let command = render_query_template(EXISTS_MESSAGE_BY_ID_TEMPLATE, &context, &state).await;
    return command_result_exists(command.as_str(), state).await;
}

pub async fn get_messages_by_room_id(room_id : &str, state : &AppState) -> Option<Vec<impl IMessage>> {
    const GET_MESSAGES_BY_ROOM_ID_TEMPLATE : &str = "database_scripts/messages/get_messages_by_account_id.sql";
    let mut context = tera::Context::new();
    context.insert("room_id", &room_id);
    
    let command = render_query_template(GET_MESSAGES_BY_ROOM_ID_TEMPLATE, &context, &state).await;
    return get_many_items_from_command(command.as_str(), state, row_to_message).await;
}

pub async fn get_messages_by_pool_id(pool_id : &str, state : &AppState) -> Option<Vec<impl IMessage>> {
    const GET_MESSAGES_BY_POOL_ID_TEMPLATE : &str = "database_scripts/messages/get_messages_by_pool_id.sql";
    let mut context = tera::Context::new();
    context.insert("pool_id", &pool_id);
    
    let command = render_query_template(GET_MESSAGES_BY_POOL_ID_TEMPLATE, &context, &state).await;
    return get_many_items_from_command(command.as_str(), state, row_to_message).await;
}

pub async fn create_message(id : &str, text_content : &str, account_id : &str, room_id : &str, pool_id : &str, creation_date : DateTime<Utc>, state : &AppState) -> () {
    let creation_date_string = creation_date.to_rfc3339();

    let mut context = tera::Context::new();
     context.insert("id", id);
     context.insert("text_content", text_content);
     context.insert("account_id", account_id);
     context.insert("room_id", room_id);
     context.insert("pool_id", pool_id);
     context.insert("creation_date", creation_date_string.as_str());
 
     const CREATE_MESSAGE_TEMPLATE : &str = "database_scripts/message/create_message.sql";
     execute_script_template_wo_return(CREATE_MESSAGE_TEMPLATE, &context, &state).await;
}

pub async fn set_message_text_content_by_id(id : &str, text_content : &str, state : &AppState) -> () {
    let mut context = tera::Context::new();
     context.insert("id", id);
     context.insert("text_content", text_content);
 
     const SET_TEXT_CONTENT_TEMPLATE : &str = "database_scripts/message/set_message_text_content.sql";
     execute_script_template_wo_return(SET_TEXT_CONTENT_TEMPLATE, &context, &state).await;
}

pub async fn delete_message_by_id(id : &str, state : &AppState) -> () {
    let mut context = tera::Context::new();
     context.insert("id", id);
 
     const DELETE_MESSAGE_TEMPLATE : &str = "database_scripts/message/delete_message_by_id.sql";
     execute_script_template_wo_return(DELETE_MESSAGE_TEMPLATE, &context, &state).await;
}