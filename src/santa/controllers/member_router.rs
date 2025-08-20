use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::controllers::ICRUDController, santa::{data_model::implementations::Member, services::{row_to_member, user_add_member_to_pool, user_create_pool}}, AppState};

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

    async fn create_object_and_return_id(obj : CreateMemberRequestData, state : &AppState) -> String {
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
}

pub fn member_router(state : &AppState) -> Router<AppState> {
    return MemberCRUDController::objects_router(state);
}