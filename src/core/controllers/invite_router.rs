use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ICRUDController}, data_model::implementations::Invite, services::{row_to_invite, user_create_invite_code}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateInviteRequestData {
    pub one_use : Option<bool>
}

pub struct InviteCRUDController {}

impl ICRUDController<CreateInviteRequestData, Invite> for InviteCRUDController {
    fn object_type_name() -> String { return String::from("invite"); }

    fn table_name() -> String { return String::from("invites"); }

    fn transform_func() -> fn(&SqliteRow) -> Invite { return row_to_invite; }

    async fn create_object_and_return_id(obj : CreateInviteRequestData, state : &AppState) -> ApiResponse {
        let one_use = obj.one_use.unwrap_or(true);
        return user_create_invite_code(one_use, state).await;
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

pub fn invite_router(state : &AppState) -> Router<AppState> {
    return InviteCRUDController::objects_router(state);
}