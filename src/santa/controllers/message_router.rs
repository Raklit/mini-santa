use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::controllers::{ApiResponse, ICRUDController}, santa::{data_model::implementations::Message, services::{row_to_message, user_send_message_to_room}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateMessageRequestData {
    pub room_id : String,
    pub account_id : String,
    pub text_content : String
}

pub struct MessageCRUDController {}

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
}

pub fn message_router(state : &AppState) -> Router<AppState> {
    return MessageCRUDController::objects_router(state);
}