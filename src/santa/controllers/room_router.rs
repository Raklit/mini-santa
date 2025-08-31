use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ICRUDController}, data_model::traits::ILocalObject, services::{IDbService, SQLiteDbService}}, santa::{data_model::implementations::Room, services::{row_to_member, row_to_room, user_create_room_for_members}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateRoomRequestData {
    pub member_mailer_id : String,
    pub member_recipient_id : String
}

pub struct RoomCRUDController {}

impl RoomCRUDController {
    async fn filter_user_rooms(state : &AppState, executor_id : &str) -> Option<Vec<Room>> {
        let db_service = SQLiteDbService::new(state);

        let executor_values = vec![executor_id];
        let executor_members = db_service.get_many_by_prop("members", "account_id", executor_values, row_to_member).await.unwrap_or(vec![]);
        
        let executor_member_ids : Vec<&str> = executor_members.iter().map(|m| {m.id()}).collect();

        let mut rooms = db_service.get_many_by_prop("rooms", "mailer_id", executor_member_ids.clone(), row_to_room).await.unwrap_or(vec![]);
        rooms.extend(db_service.get_many_by_prop("rooms", "recipient_id", executor_member_ids.clone(), row_to_room).await.unwrap_or(vec![]));
        return Some(rooms);
    }
}

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
    
    async fn check_perm_create(_state : &AppState, _executor_id : &str) -> bool {
        return true;
    }
    
    async fn filter_many(state : &AppState, executor_id : &str) -> Option<Vec<Room>> {
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
            return Self::filter_user_rooms(state, executor_id).await;
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

pub fn room_router(state : &AppState) -> Router<AppState> {
    return RoomCRUDController::objects_router(state);
}