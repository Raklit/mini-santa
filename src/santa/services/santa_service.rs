use chrono::{DateTime, Utc};
use ::rand::{seq::SliceRandom, rng};
use serde::{Deserialize, Serialize};
use crate::{core::{controllers::{ApiResponse, ApiResponseStatus, ICRUDController}, data_model::traits::{IAccountRelated, ILocalObject, IPublicUserInfo}, functions::new_id_safe, services::{escape_string, get_account_by_id, get_public_user_info_by_account_id, is_account_already_exists_by_id, IDbService, SQLiteDbService}}, santa::{data_model::{enums::{PoolState, RoomState}, traits::{IMember, IMessage, IPool, IPoolRelated, IRoom}}, services::{create_member, create_message, create_pool, create_room, delete_member_by_id, delete_message_by_id, delete_pool_by_id, delete_room_by_id, get_last_messages_by_room_id, get_member_by_id, get_member_by_pool_and_account_ids, get_members_by_pool_id, get_messages_by_pool_id, get_pool_by_id, get_room_by_id, get_rooms_by_pool_id, get_rooms_by_user, is_member_already_exists_by_id, is_member_already_exists_by_pool_and_account_ids, is_message_already_exists_by_id, is_pool_already_exists_by_id, is_room_already_exists_by_id, set_member_room_id, set_pool_state, set_wishlist_by_id}}, AppState};


pub async fn user_create_pool(name : &str, description : &str, account_id : &str, min_price : u64, max_price : u64, state : &AppState) -> ApiResponse {
    let creation_date = Utc::now();
    let new_id = new_id_safe(is_pool_already_exists_by_id, state).await;
    let pool_id = new_id.as_str();
    let db_service = SQLiteDbService::new(state);
    let name_exists = db_service.exists_by_prop("pools", "name", name).await;
    if name_exists.is_some_and(|b| {b}) {
        let err_msg = format!("Pool with name \"{name}\" already exists");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }
    create_pool(pool_id, name, description, account_id, min_price, max_price, 2000000, creation_date, PoolState::Created, state).await;
    return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(new_id).unwrap());
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MemberNickname {
    pub account_id : String,
    pub nickname : String
}

pub async fn user_get_member_nicknames_in_pool(pool_id : &str, state : &AppState) -> ApiResponse {
    let pool_exists = is_pool_already_exists_by_id(pool_id, state).await;
    if pool_exists.is_none_or(|b| {!b}) {
        let err_msg = format!("Pool with id \"{pool_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }
    
    let members_opt = get_members_by_pool_id(pool_id, state).await;
    if members_opt.is_none() {
         let err_msg = format!("Can't get members from pool with id \"{pool_id}\"");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }
    let members = members_opt.unwrap();
    let mut result = Vec::<MemberNickname>::new();
    for member in members {
        let account_id = member.account_id();
        let public_info = get_public_user_info_by_account_id(account_id, state).await.unwrap();
        let nickname = public_info.nickname(); 
        let info = MemberNickname {
            account_id : String::from(account_id),
            nickname : String::from(nickname)
        };
        result.push(info);
    }
    return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(result).unwrap());
}

pub async fn user_add_member_to_pool(account_id : &str, pool_id : &str, wishlist : &str, state : &AppState) -> ApiResponse {
    let account_exists = is_account_already_exists_by_id(account_id, state).await;
    if account_exists.is_none_or(|b| {!b}) {
        let err_msg = format!("Account with id \"{account_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }

    let pool_opt = get_pool_by_id(pool_id, state).await;
    if pool_opt.is_none() {
        let err_msg = format!("Pool with id \"{pool_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }
    let pool = pool_opt.unwrap();
    if PoolState::Open != pool.state() {
        let err_msg = format!("State of pool with id \"{pool_id}\" does not allow add new members. Members addition is available only at the open stage");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }

    let new_id = new_id_safe(is_member_already_exists_by_id, state).await;
    
    let member_exists = is_member_already_exists_by_pool_and_account_ids(pool_id, account_id, state).await;
    if member_exists {
        let err_msg = format!("User with account id \"{account_id}\" has already been added to pool with id \"{pool_id}\"");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }

    create_member(new_id.as_str(), account_id, "", pool_id, wishlist, state).await;
    return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(new_id).unwrap());
}

pub async fn user_set_member_wishlist(pool_id : &str, account_id : &str, wishlist : &str, state : &AppState) -> () {
    let member_option = get_member_by_pool_and_account_ids(pool_id, account_id, state).await;
    if member_option.is_none() { return; }
    let member = member_option.unwrap();
    let member_id = member.id();
    set_wishlist_by_id(member_id, wishlist, state).await;
}

pub async fn user_delete_member_from_pool(pool_id : &str, account_id : &str, state : &AppState) -> ApiResponse {
    let pool_opt = get_pool_by_id(pool_id, &state).await;
    if pool_opt.is_none() {
        let err_msg = format!("Pool with id \"{pool_id}\" not found");
        return ApiResponse::error_from_str(err_msg.as_str());
    }
    let pool = pool_opt.unwrap();
    if PoolState::Open != pool.state() {
        let err_msg = format!("State of pool with id \"{pool_id}\" does not allow remove members. Members removing is available only at the open stage");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap()); 
    }
    let member_option = get_member_by_pool_and_account_ids(pool_id, account_id, state).await;
    if member_option.is_none() { 
        let msg = format!("Member with account id \"{account_id}\" and pool id \"{pool_id}\" not found");
        let resp = ApiResponse::new(ApiResponseStatus::WARNING, serde_json::to_value(msg).unwrap());
        return resp;
     }
    let member = member_option.unwrap();
    let member_id = member.id();
    delete_member_by_id(member_id, state).await;
    let msg = format!("Member with account id \"{account_id}\" was successfully deleted from pool with id \"{pool_id}\"");
    let resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(msg).unwrap());
    return resp; 
}

pub async fn user_pool_state_push(pool_id : &str, state : &AppState) -> ApiResponse {
    let pool_option = get_pool_by_id(pool_id, state).await;
    if pool_option.is_none() { 
        let err_msg = format!("Pool with id \"{pool_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap()); 
    }
    let pool = pool_option.unwrap();
    let pool_state = pool.state();
    let next_pool_state_option = match pool_state {
        PoolState::Created => Some(PoolState::Open),
        PoolState::Open => Some(PoolState::Pooling),
        PoolState::Pooling => Some(PoolState::Started),
        PoolState::Started => Some(PoolState::Ended),
        _ => None 
    };
    if next_pool_state_option.is_none() {
        let err_msg = format!("Pool with id \"{pool_id}\" already ended");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap()); 
    }
    let next_pool_state = next_pool_state_option.unwrap();
    set_pool_state(pool_id, next_pool_state.clone(), state).await;
    if next_pool_state == PoolState::Pooling {
        user_make_rooms(pool_id, state).await;
        set_pool_state(pool_id, PoolState::Started, state).await;
    }
    let msg = format!("Pool with id \"{pool_id}\" changed state");
    return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(msg).unwrap());  
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

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRoomResponse {
    pub id : String,
    pub pool_id : String,
    pub recipient_id : String,
    pub room_state : RoomState,
    pub recipient_nickname : String,
    pub pool_name : String,
    pub recipient_wishlist : String
}

pub async fn user_get_rooms_by_user(account_id : &str, state : &AppState) -> ApiResponse {
    let account_exists = is_account_already_exists_by_id(account_id, state).await;
    if account_exists.is_none_or(|b| {!b}) {
        let err_msg = format!("Account with id \"{account_id}\" not found");
        return ApiResponse::error_from_str(err_msg.as_str())
    }
    
    let rooms = get_rooms_by_user(account_id, state).await.unwrap_or(vec![]);
    let mut result = Vec::<UserRoomResponse>::new();
    for room in rooms {
        let recipient_id = room.recipient_id();
        let pool_id = room.pool_id();

        let pool_opt = get_pool_by_id(pool_id, state).await;
        if pool_opt.is_none() {
            let err_msg = format!("Pool with id \"{pool_id}\" not found");
            return ApiResponse::error_from_str(err_msg.as_str());
        }
        let pool = pool_opt.unwrap();
        let pool_name = pool.name();

        let public_recipient_info_opt = get_public_user_info_by_account_id(recipient_id, state).await;
        if public_recipient_info_opt.is_none() {
            let err_msg = format!("Public info for account with id \"{recipient_id}\" not found");
            return ApiResponse::error_from_str(err_msg.as_str());
        }
        let public_recipient_info = public_recipient_info_opt.unwrap();
        let recipient_nickname = public_recipient_info.nickname();
        

        let recipient_member_opt = get_member_by_pool_and_account_ids(pool_id, recipient_id, state).await;
        if recipient_member_opt.is_none() {
            let err_msg = format!("Member with account id \"{recipient_id}\" and pool id \"{pool_id}\" not found");
            return ApiResponse::error_from_str(err_msg.as_str());
        }
        let recipient_member = recipient_member_opt.unwrap();
        let recipient_wishlist = recipient_member.wishlist();

        let temp = UserRoomResponse {
            id: String::from(room.id()),
            pool_id: String::from(pool_id),
            recipient_id: String::from(recipient_id),
            room_state: room.room_state(),
            recipient_nickname: String::from(recipient_nickname),
            pool_name: String::from(pool_name),
            recipient_wishlist: String::from(recipient_wishlist)
        };
        result.push(temp);
        
    }

    return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(result).unwrap());
}

pub async fn user_get_room_info_by_id(room_id : &str, state : &AppState) -> ApiResponse {
        let room = get_room_by_id(room_id, state).await.unwrap();
        
        let recipient_id = room.recipient_id();
        let pool_id = room.pool_id();

        let pool_opt = get_pool_by_id(pool_id, state).await;
        if pool_opt.is_none() {
            let err_msg = format!("Pool with id \"{pool_id}\" not found");
            return ApiResponse::error_from_str(err_msg.as_str());
        }
        let pool = pool_opt.unwrap();
        let pool_name = pool.name();

        let public_recipient_info_opt = get_public_user_info_by_account_id(recipient_id, state).await;
        if public_recipient_info_opt.is_none() {
            let err_msg = format!("Public info for account with id \"{recipient_id}\" not found");
            return ApiResponse::error_from_str(err_msg.as_str());
        }
        let public_recipient_info = public_recipient_info_opt.unwrap();
        let recipient_nickname = public_recipient_info.nickname();
        

        let recipient_member_opt = get_member_by_pool_and_account_ids(pool_id, recipient_id, state).await;
        if recipient_member_opt.is_none() {
            let err_msg = format!("Member with account id \"{recipient_id}\" and pool id \"{pool_id}\" not found");
            return ApiResponse::error_from_str(err_msg.as_str());
        }
        let recipient_member = recipient_member_opt.unwrap();
        let recipient_wishlist = recipient_member.wishlist();

        let result = UserRoomResponse {
            id: String::from(room.id()),
            pool_id: String::from(pool_id),
            recipient_id: String::from(recipient_id),
            room_state: room.room_state(),
            recipient_nickname: String::from(recipient_nickname),
            pool_name: String::from(pool_name),
            recipient_wishlist: String::from(recipient_wishlist)
        };
        return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(result).unwrap());
}

pub async fn user_send_message_to_room(room_id : &str, account_id : &str, text_content : &str, state : &AppState) -> ApiResponse {
    let trimmed_text = text_content.trim();
    if trimmed_text.is_empty() { 
        let err_msg = String::from("Message content is empty");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap()); 
    }
    
    let account_exists = is_account_already_exists_by_id(account_id, state).await;
    if account_exists.is_none_or(|b| {!b}) {
        let err_msg = format!("Account with id \"{account_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());  
    }

    let room_option = get_room_by_id(room_id, state).await;
    if room_option.is_none() {
        let err_msg = format!("Room with id \"{room_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());  
    }
    let room = room_option.unwrap();

    let recipient_id = room.recipient_id();
    let recipient_opt = get_member_by_id(recipient_id, state).await;
    if recipient_opt.is_none() {
        let err_msg = format!("Member with id (\"recipient_id\") \"{recipient_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }
    let recipient = recipient_opt.unwrap();

    let mailer_id = room.mailer_id();
    let mailer_opt = get_member_by_id(mailer_id, state).await;
    if mailer_opt.is_none() {
        let err_msg = format!("Member with id (\"mailer_id\") \"{mailer_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }
    let mailer = mailer_opt.unwrap();

    let mailer_acc_id = mailer.account_id();
    let recipient_acc_id = recipient.account_id();

    let mailer_acc_exists = is_account_already_exists_by_id(mailer_acc_id, state).await;
    if mailer_acc_exists.is_none_or(|b| {!b}) {
        let err_msg = format!("Member with id (\"mailer_id\") \"{mailer_id}\" account with id \"{mailer_acc_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }

    let recipient_acc_exists = is_account_already_exists_by_id(recipient_acc_id, state).await;
    if recipient_acc_exists.is_none_or(|b| {!b}) {
        let err_msg = format!("Member with id (\"recipient_id\") \"{recipient_id}\" account with id \"{recipient_acc_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }

    if !account_id.eq(mailer_acc_id) && !account_id.eq(recipient_acc_id) { 
        let err_msg = format!("Account with id \"{account_id}\" is not a member of the room with id \"{room_id}\"");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap()); 
    }

    let pool_id = room.pool_id();
    let pool_opt = get_pool_by_id(pool_id, state).await;
    if pool_opt.is_none() {
        let err_msg = format!("Room with id \"{room_id}\" pool with id \"{pool_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap()); 
    }
    let pool = pool_opt.unwrap();

    if PoolState::Started != pool.state() {
        let err_msg = format!("State of pool with id \"{pool_id}\" does not allow send messages. Sending messages is available only at the starting stage");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap()); 
    }

    let creation_date = Utc::now();
    let new_id = new_id_safe(is_message_already_exists_by_id, state).await;
    create_message(new_id.as_str(), trimmed_text, account_id, room_id, pool_id, creation_date, state).await;
    return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(new_id).unwrap());
}

pub async fn user_create_room_for_members(member_mailer_id : &str, member_recipient_id : &str, state : &AppState) -> ApiResponse{
    let member_mailer_option = get_member_by_id(member_mailer_id, state).await;
    if member_mailer_option.is_none() { 
        let err_msg = format!("Member with id (\"member_mailer_id\") \"{member_mailer_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap()); 
    }
    let member_mailer = member_mailer_option.unwrap();
    let member_recipient_option = get_member_by_id(member_recipient_id, state).await;
    if member_recipient_option.is_none() {
        let err_msg = format!("Member with id (\"member_recipient_id\") \"{member_recipient_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap()); 
    } 
    let member_recipient = member_recipient_option.unwrap();
    let same_pool = member_mailer.pool_id() == member_recipient.pool_id();
    if !same_pool {
        let err_msg = format!("Member with id (\"member_mailer_id\") \"{member_mailer_id}\" and member with id (\"member_recipient_id\") \"{member_recipient_id}\" belong to different pools");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }

    let pool_id = member_mailer.pool_id();
    let pool_option = get_pool_by_id(pool_id, state).await;
    if pool_option.is_none() {
        let err_msg = format!("Pool with id \"{pool_id}\" not found");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }
    let pool = pool_option.unwrap();
    if PoolState::Pooling != pool.state() {
        let err_msg = format!("State of pool with id \"{pool_id}\" does not allow create new rooms. Room creation is available only at the pooling stage");
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }

    let new_id = new_id_safe(is_room_already_exists_by_id, state).await;
    let room_id = new_id.as_str();
    create_room(room_id, pool_id, member_mailer.account_id(), member_recipient.account_id(), RoomState::ChoosingAGift, state).await;
    set_member_room_id(member_mailer_id, room_id, state).await;
    set_member_room_id(member_recipient_id, room_id, state).await;
    return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(new_id).unwrap());
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
    if pool_option.is_none() { return; }
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

#[derive(Serialize, Deserialize, Clone)]
pub struct UserMessageResponse {
    id : String,
    text_content : String,
    is_recipient : bool,
    creation_date : DateTime<chrono::Utc>
}

pub async fn user_get_last_messages_by_room_id(room_id : &str, limit : usize, state : &AppState) -> ApiResponse {
    let esc_room_id_string = escape_string(room_id);
    let esc_room_id = esc_room_id_string.as_str();

    let room_opt = get_room_by_id(esc_room_id, state).await;
    if room_opt.is_none() {
        let err_msg = format!("Room with id \"{esc_room_id}\" not found");
        return ApiResponse::error_from_str(err_msg.as_str());
    }
    let room = room_opt.unwrap();

    let messages = get_last_messages_by_room_id(esc_room_id, limit, state).await.unwrap_or(vec![]);
    let mut result = Vec::<UserMessageResponse>::new();
    for message in messages {
        let is_recipient = message.account_id() == room.recipient_id();
        let temp = UserMessageResponse {
            id : String::from(message.id()),
            text_content : String::from(message.text_content()),
            is_recipient : is_recipient,
            creation_date : message.creation_date() 
        };
        result.push(temp);
    }
    return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(result).unwrap());
}

pub async fn user_send_message_to_room2(room_id : &str, account_id : &str, text_content : &str, state : &AppState) -> ApiResponse {
    let esc_room_id_string = escape_string(room_id);
    let esc_room_id = esc_room_id_string.as_str();

    let room_opt = get_room_by_id(esc_room_id, state).await;
    if room_opt.is_none() {
        let err_msg = format!("Room with id \"{esc_room_id}\" not found");
        return  ApiResponse::error_from_str(err_msg.as_str());
    }
    let room = room_opt.unwrap();


    let esc_account_id_string = escape_string(account_id);
    let esc_account_id = esc_account_id_string.as_str();
    let account_exists = is_account_already_exists_by_id(esc_account_id, state).await;
    if account_exists.is_none_or(|b| {!b}) {
        let err_msg = format!("Account with id \"{esc_account_id}\" not found");
        return  ApiResponse::error_from_str(err_msg.as_str());
    }

    if room.mailer_id() != esc_account_id && room.recipient_id() != esc_account_id {
        let err_msg = format!("Account with id \"{esc_account_id}\" is not a member of room with id \"esc_room_id\"");
        return  ApiResponse::error_from_str(err_msg.as_str());
    }

    let esc_text_content_string = escape_string(text_content);
    let esc_text_content = esc_text_content_string.as_str();

    let message_content = esc_text_content.trim();
    if message_content.is_empty() {
        let err_msg = format!("Message body is empty");
        return  ApiResponse::error_from_str(err_msg.as_str());
    }

    let message_id_string = new_id_safe(is_message_already_exists_by_id, state).await;
    let message_id = message_id_string.as_str();

    let creation_date = Utc::now();

    let pool_id = room.pool_id();

    create_message(message_id, message_content, esc_account_id, esc_room_id, pool_id, creation_date, state).await;

    return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(message_id).unwrap())
}