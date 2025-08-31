use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ICRUDController}, data_model::traits::ILocalObject, services::{IDbService, SQLiteDbService}}, santa::{data_model::implementations::Message, services::{row_to_member, row_to_message, row_to_room, user_send_message_to_room}}, AppState};

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

        let is_admin = Self::is_executor_admin(state, executor_id).await;
        if is_admin {
            return db_service.get_all(Self::table_name().as_str(), Self::transform_func()).await;
        }

        let is_moderator = Self::is_executor_moderator(state, executor_id).await;
        if is_moderator {
            return db_service.get_all(Self::table_name().as_str(), Self::transform_func()).await;
        }

        let is_user = Self::is_executor_user(state, executor_id).await;
        if is_user {
            return Self::filter_user_messages(state, executor_id).await;
        }

        return None;
    }
    
    async fn check_perm_update(state : &AppState, executor_id : &str, _object_id : &str) -> bool {
        return Self::only_for_admin_or_moderator(state, executor_id).await;
    }
    
    async fn check_perm_delete(state : &AppState, executor_id : &str, _object_id : &str) -> bool {
        return Self::only_for_admin_or_moderator(state, executor_id).await;
    }
}

pub fn message_router(state : &AppState) -> Router<AppState> {
    return MessageCRUDController::objects_router(state);
}