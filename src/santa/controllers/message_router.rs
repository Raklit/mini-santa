use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ICRUDController, WhoIsExecutor}, data_model::traits::{IAccountRelated, ILocalObject}, services::{IDbService, SQLiteDbService}}, santa::{data_model::{enums::PoolState, implementations::{Message, Pool, Room}, traits::{IPool, IPoolRelated, IRoomRelated}}, services::{row_to_member, row_to_message, row_to_pool, row_to_room, user_send_message_to_room}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateMessageRequestData {
    pub room_id : String,
    pub account_id : String,
    pub text_content : String
}

pub struct MessageCRUDController {}

impl MessageCRUDController {
    async fn filter_user_messages(state : &AppState, executor_id : &str) -> Option<Vec<Message>> {
        let db_service = SQLiteDbService::new(state);

        let executor_values = vec![executor_id];
        let executor_members = db_service.get_many_by_prop("members", "account_id", executor_values, row_to_member).await.unwrap_or(vec![]);
        
        let executor_member_ids : Vec<&str> = executor_members.iter().map(|m| {m.id()}).collect();

        let mut rooms = db_service.get_many_by_prop("rooms", "mailer_id", executor_member_ids.clone(), row_to_room).await.unwrap_or(vec![]);
        rooms.extend(db_service.get_many_by_prop("rooms", "recipient_id", executor_member_ids.clone(), row_to_room).await.unwrap_or(vec![]));

        let rooms_ids : Vec<&str> = rooms.iter().map(|o| {o.id()}).collect();

        let messages = db_service.get_many_by_prop(Self::table_name().as_str(), "room_id", rooms_ids, Self::transform_func()).await.unwrap_or(vec![]);

        return Some(messages); 
    }

    async fn get_message_room_and_pool_by_message_id(state : &AppState, message_id : &str) -> (Option<Message>, Option<Room>, Option<Pool>) {
        let db_service = SQLiteDbService::new(state);

        let message_opt = db_service.get_one_by_prop(Self::table_name().as_str(), "id", message_id, Self::transform_func()).await;
        if message_opt.is_none() { return (None, None, None); }
        let message = message_opt.clone().unwrap();

        let room_opt = db_service.get_one_by_prop("rooms", "id", message.room_id(), row_to_room).await;
        let pool_opt = db_service.get_one_by_prop("pools", "id", message.pool_id(), row_to_pool).await;

        return (message_opt, room_opt, pool_opt);
    }

    async fn basic_check_owner(state : &AppState, executor_id : &str, object_id : &str) -> (Option<bool>, WhoIsExecutor) {
        let (basic_check, role) = Self::basic_check_perm(state, executor_id).await;
        if basic_check.is_some() { return (basic_check, role); }
        
        let (message_opt, room_opt, pool_opt) = Self::get_message_room_and_pool_by_message_id(state, object_id).await;
        
        if message_opt.is_none() { return (Some(false), WhoIsExecutor::NoMatter); }
        if room_opt.is_none() { return (Some(false), WhoIsExecutor::NoMatter); }
        if pool_opt.is_none() { return (Some(false), WhoIsExecutor::NoMatter); }

        let message = message_opt.unwrap();
        let pool = pool_opt.unwrap();

        let is_resource_owner = message.account_id() == executor_id;
        if is_resource_owner { return (None, WhoIsExecutor::ResourceOwner); }

        let is_pool_owner = pool.account_id() == executor_id;
        if is_pool_owner { return (None, WhoIsExecutor::PoolOwner); }

        return (None, WhoIsExecutor::Other);
    }
}

impl ICRUDController<CreateMessageRequestData, Message> for MessageCRUDController {
    fn object_type_name() -> String { return String::from("message"); }

    fn table_name() -> String { return String::from("messages"); }

    fn transform_func() -> fn(&SqliteRow) -> Message { return row_to_message; }

    async fn create_object_and_return_id(obj : CreateMessageRequestData, state : &AppState) -> ApiResponse {
        return user_send_message_to_room(obj.room_id.as_str(), obj.account_id.as_str(), obj.text_content.as_str(), state).await;
    }

    fn objects_router(_ : &AppState) -> Router<AppState> {
        return Router::new()
            .route("/", get(Self::get_objects_list_handler))
            .route("/id/{id}", get(Self::get_object_by_id_handler))
            .route("/", post(Self::create_object_handler))
            .route("/id/{id}", put(Self::update_object_by_id_handler))
            .route("/id/{id}", delete(Self::delete_object_by_id_handler));
    }
    
    async fn check_perm_create(_state : &AppState, _executor_id : &str) -> bool {
        return true;
    }
    
    async fn filter_many(state : &AppState, executor_id : &str) -> Option<Vec<Message>> {
        let db_service = SQLiteDbService::new(state);

        let is_user = Self::is_executor_user(state, executor_id).await;
        if is_user {
            return Self::filter_user_messages(state, executor_id).await;
        }

        let (is_admin_or_moderator, _) = Self::only_for_admin_or_moderator(state, executor_id).await;


        if is_admin_or_moderator {
            return db_service.get_all(Self::table_name().as_str(), Self::transform_func()).await;
        }

        return None;
    }
    
    async fn check_perm_update(state : &AppState, executor_id : &str, object_id : &str) -> bool {
        let (basic_check, role) = Self::basic_check_owner(state, executor_id, object_id).await;
        if basic_check.is_some() { return basic_check.unwrap(); }
        if role == WhoIsExecutor::Other { return false; }

        let (_, _, pool_opt) = Self::get_message_room_and_pool_by_message_id(state, object_id).await;
        if pool_opt.is_none() { return false; }
        let pool = pool_opt.unwrap();

        if role == WhoIsExecutor::ResourceOwner && pool.state() == PoolState::Started {
            return true;
        }

        return false;
    }
    
    async fn check_perm_delete(state : &AppState, executor_id : &str, object_id : &str) -> bool {
        let (basic_check, role) = Self::basic_check_owner(state, executor_id, object_id).await;
        if basic_check.is_some() { return basic_check.unwrap(); }
        if role == WhoIsExecutor::Other { return false; }

        let (_, _, pool_opt) = Self::get_message_room_and_pool_by_message_id(state, object_id).await;
        if pool_opt.is_none() { return false; }
        let pool = pool_opt.unwrap();

        if role == WhoIsExecutor::ResourceOwner && pool.state() == PoolState::Started {
            return true;
        }

        if role == WhoIsExecutor::PoolOwner && pool.state() == PoolState::Ended {
            return true;
        }

        return false;
    }
}

pub fn message_router(state : &AppState) -> Router<AppState> {
    return MessageCRUDController::objects_router(state);
}