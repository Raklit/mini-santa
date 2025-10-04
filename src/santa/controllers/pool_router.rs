use axum::{body::Body, extract::{Path, Request, State}, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::{delete, get, post, put}, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ICRUDController, WhoIsExecutor}, data_model::traits::IAccountRelated, services::{escape_string, IDbService, SQLiteDbService}}, santa::{data_model::{enums::PoolState, implementations::Pool, traits::IPool}, services::{row_to_pool, user_create_pool, user_delete_member_from_pool, user_get_member_nicknames_in_pool, user_pool_state_push}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreatePoolRequestData {
    pub name : String,
    pub description : String,
    pub account_id : Option<String>,
    pub min_price : u64,
    pub max_price : u64
}

pub struct PoolCRUDController {}

impl PoolCRUDController {
    async fn basic_check_owner(state : &AppState, executor_id : &str, object_id : &str) -> (Option<bool>, WhoIsExecutor) {
        let (basic_check, role) = Self::basic_check_perm(state, executor_id).await;
        if basic_check.is_some() { return (basic_check, role); }
        
        let db_service = SQLiteDbService::new(state);

        let pool_opt = db_service.get_one_by_prop(Self::table_name().as_str(), "id", object_id, Self::transform_func()).await;
        if pool_opt.is_none() { return (Some(true), WhoIsExecutor::NoMatter); }
        let pool = pool_opt.unwrap();

        let is_resource_owner = pool.account_id() == executor_id;
        if is_resource_owner { return (None, WhoIsExecutor::ResourceOwner); }

        return (None, WhoIsExecutor::Other);
    }

    pub async fn user_get_member_nicknames_in_pool_handler(State(state) : State<AppState>, Path(id) : Path<String>, _headers : HeaderMap, _request : Request<Body>) -> impl IntoResponse {
        let result = user_get_member_nicknames_in_pool(id.as_str(), &state).await;
        if result.is_ok() {
            return (StatusCode::OK, Json(result)).into_response()
        } else {
            return (StatusCode::BAD_REQUEST, Json(result)).into_response();
        }
    }

    pub async fn user_push_pool_state_handler(State(state) : State<AppState>, Path(id) : Path<String>, headers : HeaderMap, _request : Request<Body>) -> impl IntoResponse {
        let executor_id = headers.get("account_id").unwrap().to_str().unwrap();
        let (basic_check, role) = PoolCRUDController::basic_check_owner(&state, executor_id, id.as_str()).await;
        if basic_check.is_some_and(|b| {!b}) {
           return Self::access_denied_response().into_response();
        }

        if role == WhoIsExecutor::Admin || role == WhoIsExecutor::Moderator || role == WhoIsExecutor::NoMatter || role == WhoIsExecutor::ResourceOwner {
            let resp = user_pool_state_push(id.as_str(), &state).await;
            if resp.is_ok() {
                return (StatusCode::OK, Json(resp)).into_response();
            } else {
                return (StatusCode::BAD_REQUEST, Json(resp)).into_response();
            }
        }

        return Self::access_denied_response().into_response();
    }

    pub async fn user_delete_me_from_pool_handler(State(state) : State<AppState>, Path(id) : Path<String>, headers : HeaderMap, _request : Request<Body>) -> impl IntoResponse {
        let esc_id_string = escape_string(id.as_str());
        let pool_id= esc_id_string.as_str();
        let executor_id = headers.get("account_id").unwrap().to_str().unwrap();
        let resp = user_delete_member_from_pool(pool_id, executor_id, &state).await;
        return (StatusCode::OK, Json(resp)).into_response();
    }

    pub async fn user_delete_member_from_pool_handler(State(state) : State<AppState>, Path((id, account_id)) : Path<(String, String)>, headers : HeaderMap, _request : Request<Body>) -> impl IntoResponse {
        let esc_id_string = escape_string(id.as_str());
        let pool_id= esc_id_string.as_str();
        let esc_acc_id_string = escape_string(account_id.as_str());
        let acc_id = esc_acc_id_string.as_str();
        let executor_id = headers.get("account_id").unwrap().to_str().unwrap();
        let (basic_check, role) = PoolCRUDController::basic_check_owner(&state, executor_id, pool_id).await;
        if basic_check.is_some_and(|b| {!b}) {
            return Self::access_denied_response().into_response();
        }
        if role == WhoIsExecutor::Admin || role == WhoIsExecutor::Moderator || role == WhoIsExecutor::NoMatter || role == WhoIsExecutor::ResourceOwner {
            let resp = user_delete_member_from_pool(pool_id, acc_id, &state).await;
            return (StatusCode::OK, Json(resp)).into_response();
        }
        return Self::access_denied_response().into_response();
    }
}

impl PoolCRUDController {}

impl ICRUDController<CreatePoolRequestData, Pool> for PoolCRUDController {
    fn object_type_name() -> String { return String::from("pool"); }

    fn table_name() -> String { return String::from("pools"); }

    fn transform_func() -> fn(&SqliteRow) -> Pool { return row_to_pool; }

    async fn create_object_and_return_id(executor_id : &str, obj : CreatePoolRequestData, state : &AppState) -> ApiResponse {
        let account_id = obj.account_id.unwrap_or(String::from(executor_id));
        return user_create_pool(obj.name.as_str(), obj.description.as_str(), account_id.as_str(), obj.min_price, obj.max_price, state).await;
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
    
    async fn filter_many(state : &AppState, _executor_id : &str) -> Option<Vec<Pool>> {
        let db_service = SQLiteDbService::new(state);
        return db_service.get_all(Self::table_name().as_str(), Self::transform_func()).await;
    }
    
    async fn check_perm_update(state : &AppState, executor_id : &str, object_id : &str) -> bool {
        let (basic_check, role) = Self::basic_check_owner(state, executor_id, object_id).await;
        if basic_check.is_some() { return basic_check.unwrap(); }
        if role == WhoIsExecutor::Other { return false; }

        if role == WhoIsExecutor::ResourceOwner {
            return true;
        }

        return false;
    }
    
    async fn check_perm_delete(state : &AppState, executor_id : &str, object_id : &str) -> bool {
        let (basic_check, role) = Self::basic_check_owner(state, executor_id, object_id).await;
        if basic_check.is_some() { return basic_check.unwrap(); }
        if role == WhoIsExecutor::Other { return false; }
        
        let db_service = SQLiteDbService::new(state);
        let pool_opt = db_service.get_one_by_prop(Self::table_name().as_str(), "id", object_id, Self::transform_func()).await;
        if pool_opt.is_none() { return true; }
        let pool = pool_opt.unwrap();

        if role == WhoIsExecutor::ResourceOwner && pool.state() == PoolState::Ended {
            return true;
        }

        return false;
    }
}

pub fn pool_router(state : &AppState) -> Router<AppState> {
    let router = Router::<AppState>::new()
    .route("/id/{id}/members", get(PoolCRUDController::user_get_member_nicknames_in_pool_handler))
    .route("/id/{id}/push_state", post(PoolCRUDController::user_push_pool_state_handler))
    .route("/id/{id}/remove_me", delete(PoolCRUDController::user_delete_me_from_pool_handler))
    .route("/id/{id}/remove_member/{account_id}", delete(PoolCRUDController::user_delete_member_from_pool_handler));
    return PoolCRUDController::objects_router(state)
    .merge(router);
}