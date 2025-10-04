use sqlx::{sqlite::SqliteRow, Row};

use crate::{core::services::{IDbService, SQLiteDbService}, santa::data_model::{enums::RoomState, implementations::Room, traits::IRoom}, AppState};

pub fn row_to_room(row : &SqliteRow) -> Room {
    let id : &str = row.get("id");
    let pool_id : &str = row.get("pool_id");
    let mailer_id : &str = row.get("mailer_id");
    let recipient_id : &str = row.get("recipient_id");
    let room_state_num : u8 = row.get("room_state");
    let room_state =  RoomState::try_from(usize::from(room_state_num)).unwrap();
    return Room::new(id, pool_id, mailer_id, recipient_id, room_state)
}

pub async fn get_room_by_id(id : &str, state : &AppState) -> Option<impl IRoom> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("rooms", "id", id, row_to_room).await;
}

pub async fn get_rooms_by_pool_id(pool_id : &str, state : &AppState) -> Option<Vec<impl IRoom>> {
   let db_service = SQLiteDbService::new(state);
   return db_service.get_many_by_prop("rooms", "pool_id", vec![pool_id], row_to_room).await;
}

pub async fn get_rooms_by_account_id(account_id : &str, state : &AppState) -> Option<Vec<impl IRoom>> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_many_by_prop("rooms", "account_id", vec![account_id], row_to_room).await;
}

pub async fn get_rooms_by_user(account_id : &str, state : &AppState) -> Option<Vec<impl IRoom>> {
    let db_service = SQLiteDbService::new(state);
    let mailer_rooms = db_service.get_many_by_prop("rooms", "mailer_id", vec![account_id], row_to_room).await.unwrap_or(vec![]);
    let recipient_rooms = db_service.get_many_by_prop("rooms", "recipient_id", vec![account_id], row_to_room).await.unwrap_or(vec![]);
    let mut result = Vec::<Room>::new();
    result.extend(mailer_rooms);
    result.extend(recipient_rooms);
    return Some(result);
}

pub async fn is_room_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
   let db_service = SQLiteDbService::new(state);
   return db_service.exists_by_prop("rooms", "id", id).await;
}

pub async fn create_room(id : &str, pool_id : &str, mailer_id : &str, recipient_id : &str, room_state : RoomState, state : &AppState) -> () {
    let room_state_num = room_state as usize;
    let room_state_string = room_state_num.to_string();
    let room_state_str = room_state_string.as_str();

    let db_service = SQLiteDbService::new(state);
    let _ = db_service.insert("rooms",
    vec!["id", "pool_id", "mailer_id", "recipient_id", "room_state"],
    vec![vec![id, pool_id, mailer_id, recipient_id, room_state_str]]).await;
}

pub async fn set_room_state_by_id(id : &str, room_state : RoomState, state : &AppState) -> () {
    let room_state_num = room_state as usize;
    let room_state_string = room_state_num.to_string();
    let room_state_str = room_state_string.as_str();

    let db_service = SQLiteDbService::new(state);
    db_service.update("rooms", "id", id, vec!["room_state"], vec![room_state_str]).await;
}

pub async fn delete_room_by_id(id : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    db_service.delete_one_by_prop("rooms", "id", id).await;
}