use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::controllers::{ApiResponse, ICRUDController}, santa::{data_model::implementations::Pool, services::{row_to_pool, user_create_pool}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreatePoolRequestData {
    pub name : String,
    pub description : String,
    pub account_id : String,
    pub min_price : u64,
    pub max_price : u64
}

pub struct PoolCRUDController {}

impl ICRUDController<CreatePoolRequestData, Pool> for PoolCRUDController {
    fn object_type_name() -> String { return String::from("pool"); }

    fn table_name() -> String { return String::from("pools"); }

    fn transform_func() -> fn(&SqliteRow) -> Pool { return row_to_pool; }

    async fn create_object_and_return_id(obj : CreatePoolRequestData, state : &AppState) -> ApiResponse {
        return user_create_pool(obj.name.as_str(), obj.description.as_str(), obj.account_id.as_str(), obj.min_price, obj.max_price, state).await;
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

pub fn pool_router(state : &AppState) -> Router<AppState> {
    return PoolCRUDController::objects_router(state);
}