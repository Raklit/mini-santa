use chrono::Utc;

use crate::{core::{data_model::traits::ILocalObject, functions::new_id_safe, services::is_account_already_exists_by_id}, santa::{data_model::{enums::PoolState, traits::{IPool, IPoolRelated, IRoom}}, services::{create_member, create_message, create_pool, delete_member_by_id, delete_message_by_id, delete_pool_by_id, delete_room_by_id, get_member_by_pool_and_account_ids, get_members_by_pool_id, get_messages_by_pool_id, get_messages_by_room_id, get_pool_by_id, get_room_by_id, get_rooms_by_pool_id, is_member_already_exists_by_id, is_member_already_exists_by_pool_and_account_ids, is_message_already_exists_by_id, is_pool_already_exists_by_id, is_room_already_exists_by_id, set_pool_state, set_wishlist_by_id}}, AppState};

pub async fn user_create_pool(name : &str, description : &str, account_id : &str, min_price : u64, max_price : u64, is_creator_involved : bool, state : &AppState) -> () {
    let creation_date = Utc::now();
    let new_id = new_id_safe(is_pool_already_exists_by_id, state).await;
    create_pool(&new_id.as_str(), name, description, account_id, min_price, max_price, is_creator_involved, u64::MAX, creation_date, PoolState::Created, state).await;
}

pub async fn user_add_member_to_pool(account_id : &str, pool_id : &str, state : &AppState) -> () {
    let is_already_exists = is_member_already_exists_by_pool_and_account_ids(pool_id, account_id, state).await;
    if is_already_exists { return; }
    let new_id = new_id_safe(is_member_already_exists_by_id, state).await;
    create_member(new_id.as_str(), account_id, "", pool_id, "", state).await;
}

pub async fn user_set_member_wishlist(pool_id : &str, account_id : &str, wishlist : &str, state : &AppState) -> () {
    let member_option = get_member_by_pool_and_account_ids(pool_id, account_id, state).await;
    if member_option.is_none() { return; }
    let member = member_option.unwrap();
    let member_id = member.id();
    set_wishlist_by_id(member_id, wishlist, state).await;
}

pub async fn user_delete_member_from_pool(pool_id : &str, account_id : &str, state : &AppState) -> () {
    let member_option = get_member_by_pool_and_account_ids(pool_id, account_id, state).await;
    if member_option.is_none() { return; }
    let member = member_option.unwrap();
    let member_id = member.id();
    delete_member_by_id(member_id, state).await; 
}

pub async fn user_pool_state_push(pool_id : &str, state : &AppState) {
    let pool_option = get_pool_by_id(pool_id, state).await;
    if pool_option.is_none() { return; }
    let pool = pool_option.unwrap();
    let pool_state = pool.state();
    let next_pool_state_option = match pool_state {
        PoolState::Created => Some(PoolState::Started),
        PoolState::Started => Some(PoolState::Pooling),
        PoolState::Pooling => Some(PoolState::Ended),
        _ => None 
    };
    if next_pool_state_option.is_none() { return; }
    let next_pool_state = next_pool_state_option.unwrap();
    set_pool_state(pool_id, next_pool_state, state).await;
}

pub async fn user_delete_pool(pool_id : &str, state : &AppState) -> () {
    
    // delete rooms
    let room_option = get_rooms_by_pool_id(pool_id, state).await;
    if room_option.is_some() {
        let rooms = room_option.unwrap();
        for room in rooms {
            let room_id = room.id();
            delete_room_by_id(room_id, state).await;
        }
    }

    // delete members
    let members_option = get_members_by_pool_id(pool_id, state).await;
    if members_option.is_some() {
        let members = members_option.unwrap();
        for member in members {
            let member_id = member.id();
            delete_member_by_id(member_id, state).await;
        }
    }

    // delete messages
    let messages_option = get_messages_by_pool_id(pool_id, state).await;
    if messages_option.is_some() {
        let messages = messages_option.unwrap();
        for message in messages {
            let message_id = message.id();
            delete_message_by_id(message_id, state).await;
        }
    }
    
    // delete pool 
    delete_pool_by_id(pool_id, state).await;
}

pub async fn user_send_message_to_room(room_id : &str, account_id : &str, text_content : &str, state : &AppState) -> () {
    let trimmed_text = text_content.trim();
    if trimmed_text.is_empty() { return; }
    
    let account_exists = is_account_already_exists_by_id(account_id, state).await;
    if !account_exists { return; }
    let room_option = get_room_by_id(room_id, state).await;
    if room_option.is_none() { return; }
    let room = room_option.unwrap();

    let recipient_id = room.recipient_id();
    let mailer_id = room.mailer_id();
    if !account_id.eq(mailer_id) && !account_id.eq(recipient_id) { return; }

    let pool_id = room.pool_id();
    let pool_exists = is_pool_already_exists_by_id(pool_id, state).await;
    if !pool_exists { return; }

    let creation_date = Utc::now();
    let new_id = new_id_safe(is_message_already_exists_by_id, state).await;
    create_message(new_id.as_str(), trimmed_text, account_id, room_id, pool_id, creation_date, state).await;
}