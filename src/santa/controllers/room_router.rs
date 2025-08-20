use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::controllers::{ApiResponse, ICRUDController}, santa::{data_model::implementations::Room, services::{row_to_room, user_create_room_for_members}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateRoomRequestData {
    pub member_mailer_id : String,
    pub member_recipient_id : String
}

pub struct RoomCRUDController {}

impl ICRUDController<CreateRoomRequestData, Room> for RoomCRUDController {
    fn object_type_name() -> String { return String::from("room"); }

    fn table_name() -> String { return String::from("rooms"); }

    fn transform_func() -> fn(&SqliteRow) -> Room { return row_to_room; }

    async fn create_object_and_return_id(obj : CreateRoomRequestData, state : &AppState) -> ApiResponse {
        return user_create_room_for_members(obj.member_mailer_id.as_str(), obj.member_recipient_id.as_str(), state).await;
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

pub fn room_router(state : &AppState) -> Router<AppState> {
    return RoomCRUDController::objects_router(state);
}