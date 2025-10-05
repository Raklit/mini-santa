use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::{query, sqlite::SqliteRow, Executor, Row};

use crate::{core::services::{escape_string, IDbService, SQLiteDbService}, santa::data_model::{implementations::Message, traits::IMessage}, AppState};

pub fn row_to_message(row : &SqliteRow) -> Message {
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
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("messages", "id", id, row_to_message).await;
}

pub async fn get_messages_by_account_id(account_id : &str, state : &AppState) -> Option<Vec<impl IMessage>> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_many_by_prop("messages", "account_id", vec![account_id], row_to_message).await;
}

pub async fn is_message_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("messages", "id", id).await;
}

pub async fn get_messages_by_room_id(room_id : &str, state : &AppState) -> Option<Vec<impl IMessage>> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_many_by_prop("messages", "room_id", vec![room_id], row_to_message).await;
}

pub async fn get_messages_by_pool_id(pool_id : &str, state : &AppState) -> Option<Vec<impl IMessage>> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_many_by_prop("messages", "pool_id", vec![pool_id], row_to_message).await;
}

pub async fn create_message(id : &str, text_content : &str, account_id : &str, room_id : &str, pool_id : &str, creation_date : DateTime<Utc>, state : &AppState) -> () {
    let creation_date_string = creation_date.to_rfc3339();
    let creation_date_str = creation_date_string.as_str();

    let db_service = SQLiteDbService::new(state);
    db_service.insert("messages", 
    vec!["id", "text_content", "account_id", "room_id", "pool_id", "creation_date"],
    vec![vec![id, text_content, account_id, room_id, pool_id, creation_date_str]]).await;
}

pub async fn set_message_text_content_by_id(id : &str, text_content : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    db_service.update("messages", "id", id, vec!["text_content"], vec![text_content]).await;
}

pub async fn delete_message_by_id(id : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    return db_service.delete_one_by_prop("messages", "id", id).await;
}

pub async fn get_last_messages_by_room_id(room_id : &str, limit : usize, state : &AppState) -> Option<Vec<impl IMessage>> {
    let esc_room_id_string = escape_string(room_id);
    let esc_room_id = esc_room_id_string.as_str();

    let query = format!("SELECT * FROM messages WHERE room_id = \"{esc_room_id}\" ORDER BY creation_date DESC LIMIT {limit}" );
    let conn = state.db.lock().await;
    let query_result = match conn.fetch_all(query.as_str()).await {
        Ok(o) => Some(o),
        Err(_) => None
    };
    if query_result.is_none() { return None; }

    let rows = query_result.unwrap();
    let mut objs : Vec<Message> = rows.iter().map(row_to_message).collect();
    objs.reverse();
    
    return Some(objs);

}