use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ICRUDController, WhoIsExecutor}, data_model::traits::{IAccountRelated, ILocalObject}, services::{IDbService, SQLiteDbService}}, santa::{data_model::{enums::PoolState, implementations::{Pool, Room}, traits::{IPool, IPoolRelated, IRoom}}, services::{row_to_member, row_to_pool, row_to_room, user_create_room_for_members}}, AppState};

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

    async fn basic_check_owner(state : &AppState, executor_id : &str, object_id : &str) -> (Option<bool>, WhoIsExecutor) {
        let (basic_check, role) = Self::basic_check_perm(state, executor_id).await;
        if basic_check.is_some() { return (basic_check, role); }
        
        let db_service = SQLiteDbService::new(state);

        let room_opt = db_service.get_one_by_prop(Self::table_name().as_str(), "id", object_id, Self::transform_func()).await;
        if room_opt.is_none() { return (Some(true), WhoIsExecutor::NoMatter); }
        let room = room_opt.unwrap();

        let mailer_opt = db_service.get_one_by_prop("members", "mailer_id", room.mailer_id(), row_to_member).await;
        if mailer_opt.is_none() { return (Some(false), WhoIsExecutor::NoMatter); }
        let mailer = mailer_opt.unwrap();

        let recipient_opt = db_service.get_one_by_prop("members", "recipient_id", room.recipient_id(), row_to_member).await;
        if recipient_opt.is_none() { return (Some(false), WhoIsExecutor::NoMatter); }
        let recipient = recipient_opt.unwrap();
        
        let is_resource_owner = mailer.account_id() == executor_id || recipient.account_id() == executor_id;
        if is_resource_owner { return (None, WhoIsExecutor::ResourceOwner); }

        let pool_opt = db_service.get_one_by_prop("pools", "id", room.pool_id(), row_to_pool).await;
        if pool_opt.is_none() { return (Some(false), WhoIsExecutor::NoMatter); }
        let pool = pool_opt.unwrap();

        let is_pool_owner = pool.account_id() == executor_id;
        if is_pool_owner { return (None, WhoIsExecutor::PoolOwner); }

        return (None, WhoIsExecutor::Other);
    }

    async fn get_pool_by_room_id(state : &AppState, room_id : &str) -> Option<Pool> {
        let db_service = SQLiteDbService::new(state);
        
        let room_opt = db_service.get_one_by_prop(Self::table_name().as_str(), "id", room_id, Self::transform_func()).await;
        if room_opt.is_none() { return None; }
        let room = room_opt.unwrap();
        
        let pool_opt = db_service.get_one_by_prop("pools", "id", room.pool_id(), row_to_pool).await;
        if pool_opt.is_none() { return None; }
        let pool = pool_opt.unwrap();
        
        return Some(pool);
    }
}

impl ICRUDController<CreateRoomRequestData, Room> for RoomCRUDController {
    fn object_type_name() -> String { return String::from("room"); }

    fn table_name() -> String { return String::from("rooms"); }

    fn transform_func() -> fn(&SqliteRow) -> Room { return row_to_room; }

    async fn create_object_and_return_id(executor_id : &str, obj : CreateRoomRequestData, state : &AppState) -> ApiResponse {
        let mailer_id = obj.member_mailer_id.as_str();
        let recipient_id = obj.member_recipient_id.as_str();

        let (basic_check, _) = Self::basic_check_perm(state, executor_id).await;
        if basic_check.is_some_and(|b| {b}) {
            return user_create_room_for_members(mailer_id, recipient_id, state).await;
        }

        let db_service = SQLiteDbService::new(state);

        let mailer_opt = db_service.get_one_by_prop("members", "id", mailer_id, row_to_member).await;
        if mailer_opt.is_none() {
            let err_msg = format!("Mailer (object type \"member\") with id \"{mailer_id}\" not found");
            return ApiResponse::error_from_str(err_msg.as_str());
        }
        let mailer = mailer_opt.unwrap();


        let recipient_opt = db_service.get_one_by_prop("members", "id", recipient_id, row_to_member).await;
        if recipient_opt.is_none() {
            let err_msg = format!("Recipient (object type \"recipient\") with id \"{recipient_id}\" not found");
            return ApiResponse::error_from_str(err_msg.as_str());
        }
        let recipient = recipient_opt.unwrap();

        if mailer.pool_id() != recipient.pool_id() {
            let err_msg = format!("Mailer (object with type \"member\") with id \"{mailer_id}\" and recipient (object with type \"member\") with id \"{recipient_id}\" belong to different pools");
            return ApiResponse::error_from_str(err_msg.as_str());
        }

        let pool_id = mailer.pool_id();
        let pool_opt = db_service.get_one_by_prop("pools", "id", pool_id, row_to_pool).await;
        if pool_opt.is_none() {
            let err_msg = format!("Pool with id \"{pool_id}\" not found");
            return ApiResponse::error_from_str(err_msg.as_str()); 
        }
        let pool = pool_opt.unwrap();

        if pool.account_id() != executor_id {
            let err_msg = format!("Only pool owner can create rooms in pool");
            return ApiResponse::error_from_str(err_msg.as_str()); 
        }

        if pool.state() != PoolState::Pooling {
            let err_msg = format!("State of pool with id \"{pool_id}\" does not allow create new rooms. Room creation is available only at the pooling stage");
            return ApiResponse::error_from_str(err_msg.as_str())
        }

        return user_create_room_for_members(mailer_id, recipient_id, state).await;
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

        let is_user = Self::is_executor_user(state, executor_id).await;
        if is_user {
            return Self::filter_user_rooms(state, executor_id).await;
        }

        let (is_admin_or_moderator, _) = Self::only_for_admin_or_moderator(state, executor_id).await;
        if is_admin_or_moderator {
            return db_service.get_all(Self::table_name().as_str(), Self::transform_func()).await;
        }

        return None;
    }
    
    async fn check_perm_update(state : &AppState, executor_id : &str, object_id : &str) -> bool {
        let (basic_check, role) = Self::basic_check_owner(state, executor_id, object_id).await;
        if basic_check.is_some() {return basic_check.unwrap(); }
        if role == WhoIsExecutor::Other { return false; }

        let pool_opt = Self::get_pool_by_room_id(state, object_id).await;
        if pool_opt.is_none() { return false; }
        let pool = pool_opt.unwrap();

        if role == WhoIsExecutor::ResourceOwner && pool.state() == PoolState::Started {
            return true;
        }
        
        return false;
    }
    
    async fn check_perm_delete(state : &AppState, executor_id : &str, object_id : &str) -> bool {
        let (basic_check, role) = Self::basic_check_owner(state, executor_id, object_id).await;
        if basic_check.is_some() {return basic_check.unwrap(); }
        if role == WhoIsExecutor::Other { return false; }

        let pool_opt = Self::get_pool_by_room_id(state, object_id).await;
        if pool_opt.is_none() { return false; }
        let pool = pool_opt.unwrap();

        if role == WhoIsExecutor::PoolOwner && pool.state() == PoolState::Ended {
            return true;
        } 
        
        return false;
    }
}

pub fn room_router(state : &AppState) -> Router<AppState> {
    return RoomCRUDController::objects_router(state);
}