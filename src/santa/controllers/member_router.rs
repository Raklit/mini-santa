use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ICRUDController}, services::{IDbService, SQLiteDbService}}, santa::{data_model::implementations::Member, services::{row_to_member, user_add_member_to_pool}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateMemberRequestData {
    pub account_id : String,
    pub pool_id : String,
    pub wishlist : Option<String>
}

pub struct MemberCRUDController {}

impl ICRUDController<CreateMemberRequestData, Member> for MemberCRUDController {
    fn object_type_name() -> String { return String::from("member"); }

    fn table_name() -> String { return String::from("members"); }

    fn transform_func() -> fn(&SqliteRow) -> Member { return row_to_member; }

    async fn create_object_and_return_id(obj : CreateMemberRequestData, state : &AppState) -> ApiResponse {
        let wishlist = obj.wishlist.unwrap_or(String::new());
        return user_add_member_to_pool(obj.account_id.as_str(), obj.pool_id.as_str(), wishlist.as_str(), state).await;
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
    
    async fn filter_many(state : &AppState, executor_id : &str) -> Option<Vec<Member>> {
        let db_service = SQLiteDbService::new(state);

        let is_user = Self::is_executor_user(state, executor_id).await;
        if is_user {
            let values = vec![executor_id];
            return db_service.get_many_by_prop(Self::table_name().as_str(), "account_id", values, Self::transform_func()).await;
        }

        let is_admin = Self::is_executor_admin(state, executor_id).await;
        if is_admin {
            return  db_service.get_all(Self::table_name().as_str(), Self::transform_func()).await;
        }

        let is_moderator = Self::is_executor_moderator(state, executor_id).await;
        if is_moderator {
            return  db_service.get_all(Self::table_name().as_str(), Self::transform_func()).await;
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

pub fn member_router(state : &AppState) -> Router<AppState> {
    return MemberCRUDController::objects_router(state);
}