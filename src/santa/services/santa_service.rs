use chrono::Utc;
use ::rand::{seq::SliceRandom, rng};

use crate::{core::{data_model::traits::{IAccountRelated, ILocalObject}, functions::new_id_safe, services::is_account_already_exists_by_id}, santa::{data_model::{enums::{PoolState, RoomState}, traits::{IPool, IPoolRelated, IRoom}}, services::{create_member, create_message, create_pool, create_room, delete_member_by_id, delete_message_by_id, delete_pool_by_id, delete_room_by_id, get_member_by_id, get_member_by_pool_and_account_ids, get_members_by_pool_id, get_messages_by_pool_id, get_messages_by_room_id, get_pool_by_id, get_room_by_id, get_rooms_by_pool_id, is_member_already_exists_by_id, is_member_already_exists_by_pool_and_account_ids, is_message_already_exists_by_id, is_pool_already_exists_by_id, is_room_already_exists_by_id, set_member_room_id, set_pool_state, set_wishlist_by_id}}, AppState};


pub async fn user_create_pool(name : &str, description : &str, account_id : &str, min_price : u64, max_price : u64, state : &AppState) -> String {
    let creation_date = Utc::now();
    let new_id = new_id_safe(is_pool_already_exists_by_id, state).await;
    let pool_id = new_id.as_str();
    create_pool(pool_id, name, description, account_id, min_price, max_price, 2000000, creation_date, PoolState::Created, state).await;
    return String::from(pool_id);
}

pub async fn user_add_member_to_pool(account_id : &str, pool_id : &str, wishlist_opt : Option<String>, state : &AppState) -> String {
    let new_id = new_id_safe(is_member_already_exists_by_id, state).await;
    let wishlist = wishlist_opt.unwrap_or(String::new());
    create_member(new_id.as_str(), account_id, "", pool_id, wishlist.as_str(), state).await;
    return new_id;
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
    if account_exists.is_none_or(|b| {!b}) { return; }
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

async fn user_create_room_for_members(member_mailer_id : &str, member_recipient_id : &str, state : &AppState) -> () {
    let member_mailer_option = get_member_by_id(member_mailer_id, state).await;
    if member_mailer_option.is_none() { return; }
    let member_mailer = member_mailer_option.unwrap();
    let member_recipient_option = get_member_by_id(member_recipient_id, state).await;
    if member_recipient_option.is_none() { return; }
    let member_recipient = member_recipient_option.unwrap();
    let same_pool = member_mailer.pool_id() == member_recipient.pool_id();
    if !same_pool { return; }

    let pool_id = member_mailer.pool_id();
    let pool_option = get_pool_by_id(pool_id, state).await;
    if pool_option.is_none() { return; }
    let pool = pool_option.unwrap();
    if PoolState::Pooling != pool.state() { return; }

    let new_id = new_id_safe(is_room_already_exists_by_id, state).await;
    let room_id = new_id.as_str();
    create_room(room_id, pool_id, member_mailer.account_id(), member_recipient.account_id(), RoomState::ChosingAGift, state).await;
    set_member_room_id(member_mailer_id, room_id, state).await;
    set_member_room_id(member_recipient_id, room_id, state).await;
}

fn make_pairs(vector : &Vec<&str>) -> Vec<[String; 2]> {
   let mut mix = vector.clone();
   mix.shuffle(&mut rng());
   mix.push(mix[0]);
   let mut result : Vec<[String; 2]> = Vec::new();
   let n = mix.len();
   for i in 0..(n - 1) {
        let temp : [String; 2] = [String::from(mix[i]), String::from(mix[i+1])];
        result.push(temp);
   }
   return result;
}

pub async fn user_make_rooms(pool_id : &str, state : &AppState) -> () {
    let pool_option = get_pool_by_id(pool_id, state).await;
    if !pool_option.is_none() { return; }
    let pool = pool_option.unwrap();
    if PoolState::Pooling != pool.state() { return; }
    let members_option = get_members_by_pool_id(pool_id, state).await;
    if members_option.is_none() { return; }
    let members = members_option.unwrap();
    let member_ids : Vec<&str> = members.iter().map(|m| -> &str {m.id()}).collect();
    let pairs = make_pairs(&member_ids);
    for pair in pairs {
        let member_mailer_id = pair[0].as_str();
        let member_recipient_id = pair[1].as_str();
        user_create_room_for_members(member_mailer_id, member_recipient_id, state).await;
    }
}