use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ICRUDController}, data_model::implementations::Invite, services::{row_to_invite, user_create_invite_code, IDbService, SQLiteDbService}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateInviteRequestData {
    pub invite_code : Option<String>,
    pub one_use : Option<bool>
}

pub struct InviteCRUDController {}

impl ICRUDController<CreateInviteRequestData, Invite> for InviteCRUDController {
    fn object_type_name() -> String { return String::from("invite"); }

    fn table_name() -> String { return String::from("invites"); }

    fn transform_func() -> fn(&SqliteRow) -> Invite { return row_to_invite; }

    async fn create_object_and_return_id(_executor_id : &str, obj : CreateInviteRequestData, state : &AppState) -> ApiResponse {        
        let invite_code = obj.invite_code.unwrap_or(String::new());
        let one_use = obj.one_use.unwrap_or(true);
        return user_create_invite_code(invite_code.as_str() ,one_use, state).await;
    }

    fn objects_router(_ : &AppState) -> Router<AppState> {
        return Router::new()
            .route("/", get(Self::get_objects_list_handler))
            .route("/id/{id}", get(Self::get_object_by_id_handler))
            .route("/", post(Self::create_object_handler))
            .route("/id/{id}", put(Self::update_object_by_id_handler))
            .route("/id/{id}", delete(Self::delete_object_by_id_handler));
    }
    
    async fn check_perm_create(state : &AppState, executor_id : &str) -> bool {
        let (admin_or_moderator, _) = Self::only_for_admin_or_moderator(state, executor_id).await;
        return admin_or_moderator;
    }
    
    async fn check_perm_get(state : &AppState, executor_id : &str, _object_id : &str) -> bool {
        return Self::check_perm_create(state, executor_id).await;
    }
    
    async fn filter_many(state : &AppState, executor_id : &str) -> Option<Vec<Invite>> {
        let is_user = Self::is_executor_user(state, executor_id).await;
        if is_user { return None; }

        let (admin_or_moderator, _) = Self::only_for_admin_or_moderator(state, executor_id).await;
        if admin_or_moderator {
            let db_service = SQLiteDbService::new(state);
            return  db_service.get_all(Self::table_name().as_str(), Self::transform_func()).await;
        }

        return None;
    }
    
    async fn check_perm_update(state : &AppState, executor_id : &str, _object_id : &str) -> bool {
        return Self::check_perm_create(state, executor_id).await;
    }
    
    async fn check_perm_delete(state : &AppState, executor_id : &str, _object_id : &str) -> bool {
        return Self::check_perm_create(state, executor_id).await;
    }
}

pub fn invite_router(state : &AppState) -> Router<AppState> {
    return InviteCRUDController::objects_router(state);
}