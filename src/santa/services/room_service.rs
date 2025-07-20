use sqlx::{sqlite::SqliteRow, Row};

use crate::{core::functions::{execute_script_template_wo_return, get_many_items_from_command, get_one_item_from_command, render_query_template}, santa::data_model::{enums::RoomState, implementations::Room, traits::IRoom}, AppState};

fn row_to_room(row : &SqliteRow) -> Room {
    let id : &str = row.get("id");
    let pool_id : &str = row.get("pool_id");
    let mailer_id : &str = row.get("mailer_id");
    let recipient_id : &str = row.get("recipient_id");
    let room_state_num : u8 = row.get("room_state");
    let room_state =  RoomState::try_from(usize::from(room_state_num)).unwrap();
    return Room::new(id, pool_id, mailer_id, recipient_id, room_state)
}

pub async fn get_room_by_id(id : &str, state : &AppState) -> Option<impl IRoom> {
    const GET_ROOM_BY_ID_TEMPLATE : &str = "database_scripts/room/get_room_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(GET_ROOM_BY_ID_TEMPLATE, &context, &state).await;
    return get_one_item_from_command(command.as_str(), state, row_to_room).await;
}

pub async fn get_rooms_by_pool_id(pool_id : &str, state : &AppState) -> Option<Vec<impl IRoom>> {
    const GET_ROOMS_BY_POOL_ID_TEMPLATE : &str = "database_scripts/room/get_rooms_by_pool_id.sql";
    let mut context = tera::Context::new();
    context.insert("pool_id", &pool_id);
    
    let command = render_query_template(GET_ROOMS_BY_POOL_ID_TEMPLATE, &context, &state).await;
    return get_many_items_from_command(command.as_str(), state, row_to_room).await;
}

pub async fn get_rooms_by_account_id(account_id : &str, state : &AppState) -> Option<Vec<impl IRoom>> {
    const GET_ROOMS_BY_ACCOUNT_ID_TEMPLATE : &str = "database_scripts/room/get_rooms_by_account_id.sql";
    let mut context = tera::Context::new();
    context.insert("account_id", &account_id);
    
    let command = render_query_template(GET_ROOMS_BY_ACCOUNT_ID_TEMPLATE, &context, &state).await;
    return get_many_items_from_command(command.as_str(), state, row_to_room).await;
}


pub async fn create_room(id : &str, pool_id : &str, mailer_id : &str, recipient_id : &str, room_state : RoomState, state : &AppState) -> () {
    let room_state_num = room_state as usize;

    let mut context = tera::Context::new();
     context.insert("id", id);
     context.insert("pool_id", pool_id);
     context.insert("mailer_id", mailer_id);
     context.insert("recipient_id", recipient_id);
     context.insert("room_state", &room_state_num);
 
     const CREATE_ROOM_TEMPLATE : &str = "database_scripts/room/create_room.sql";
     execute_script_template_wo_return(CREATE_ROOM_TEMPLATE, &context, &state).await;
}

pub async fn set_room_state_by_id(id : &str, room_state : RoomState, state : &AppState) -> () {
    let room_state_num = room_state as usize;
    
    let mut context = tera::Context::new();
     context.insert("id", id);
     context.insert("room_state", &room_state_num);
 
     const SET_ROOM_STATE_TEMPLATE : &str = "database_scripts/room/set_room_state.sql";
     execute_script_template_wo_return(SET_ROOM_STATE_TEMPLATE, &context, &state).await;
}

pub async fn delete_room_by_id(id : &str, state : &AppState) -> () {
    let mut context = tera::Context::new();
     context.insert("id", id);
 
     const DELETE_ROOM_TEMPLATE : &str = "database_scripts/room/delete_room_by_id.sql";
     execute_script_template_wo_return(DELETE_ROOM_TEMPLATE, &context, &state).await;
}