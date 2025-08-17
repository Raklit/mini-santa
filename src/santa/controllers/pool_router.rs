use std::collections::HashMap;

use axum::{body::Body, extract::{Path, Request, State}, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::{delete, get, post, put}, Json, Router};
use serde::{Serialize, Deserialize};

use crate::{core::{controllers::{ApiResponse, ApiResponseStatus}, services::{IDbService, SQLiteDbService}}, santa::{data_model::implementations::Pool, services::{row_to_pool, user_create_pool}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreatePoolRequestData {
    pub name : String,
    pub description : String,
    pub account_id : String,
    pub min_price : u64,
    pub max_price : u64
}

pub async fn get_pool_list_handler(State(state) : State<AppState>, headers : HeaderMap, request : Request<Body>) -> impl IntoResponse {
    let db_service = SQLiteDbService::new(&state);
    let pools_opt = db_service.get_all("pools", row_to_pool).await;
    let pools = pools_opt.unwrap_or(vec![]);
    let resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(pools).unwrap());
    return (StatusCode::OK, Json(resp)).into_response();
}

pub async fn get_pool_by_id_handler(State(state) : State<AppState>, Path(id) : Path<String>, headers : HeaderMap, request : Request<Body>) -> impl IntoResponse {
    let db_service = SQLiteDbService::new(&state);
    let pool_opt = db_service.get_one_by_prop("pools", "id", id.as_str(), row_to_pool).await;
    let resp : ApiResponse;
    if pool_opt.is_none() { 
        let err_msg = format!("Pool with id \"{id}\" not found");
        resp = ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
        return (StatusCode::NOT_FOUND, Json(resp)).into_response(); 
    }
    let pool = pool_opt.unwrap();
    resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(pool).unwrap());
    return (StatusCode::OK, Json(resp)).into_response();
}

pub async fn create_pool_handler(State(state) : State<AppState>, Json(json) : Json<CreatePoolRequestData>) -> impl IntoResponse {
    let pool_id = user_create_pool(json.name.as_str(), json.description.as_str(), json.account_id.as_str(), json.min_price, json.max_price.clone(), &state).await;
    return (StatusCode::CREATED, pool_id).into_response();
}

pub async fn update_pool_by_id_handler(State(state) : State<AppState>, Path(id) : Path<String>, headers : HeaderMap, Json(json) : Json<HashMap<String, serde_json::Value>>) -> impl IntoResponse {
    let db_service = SQLiteDbService::new(&state);
    let pool_exists = db_service.exists_by_prop("pools", "id", id.as_str()).await;
    let resp : ApiResponse;
    if pool_exists.is_none_or(|b| {!b}) { 
        let err_msg = format!("Pool with id \"{id}\" not found");
        resp = ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
        return (StatusCode::NOT_FOUND, Json(resp)).into_response(); 
    }

    let json_c = json.clone();
    
    let props_string : Vec<String> = json.into_keys().collect();
    let props : Vec<&str> = props_string.iter().map(|s| {s.as_str()}).collect();

    let values_val : Vec<serde_json::Value> = json_c.into_values().collect();
    let values_opt : Vec<Option<&str>> = values_val.iter().map(| i |{ i.as_str() }).collect();
    if values_opt.iter().any(| v| {v.is_none()}) {
        let err_msg = format!("Some JSON fields in request body not a strings");
        resp = ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
        return (StatusCode::BAD_REQUEST, Json(resp)).into_response();
    }
    let values : Vec<&str> = values_opt.iter().map(|s| {s.unwrap()}).collect();
    
    let _ = db_service.update("pools", "id", id.as_str(), props, values).await;

    let pool_opt = db_service.get_one_by_prop("pools", "id", id.as_str(), row_to_pool).await;
    if pool_opt.is_none() { 
        let err_msg = format!("Pool with id \"{id}\" not found");
        resp = ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
        return (StatusCode::NOT_FOUND, Json(resp)).into_response(); 
    }
    let pool = pool_opt.unwrap();

    resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(pool).unwrap());
    return (StatusCode::OK, Json(resp)).into_response();
}

pub async fn delete_pool_by_id_handler(State(state) : State<AppState>, Path(id) : Path<String>, headers : HeaderMap, request : Request<Body>) -> impl IntoResponse {
    let db_service = SQLiteDbService::new(&state);
    let _ = db_service.delete_one_by_prop("pools", "id", id.as_str()).await;
    let msg = format!("Pool with id \"{id}\" successfully deleted");
    let resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(msg).unwrap());
    return (StatusCode::OK, Json(resp)).into_response();
}

pub fn pool_router(state : AppState) -> Router<AppState> {
    return Router::new()
    .route("/", get(get_pool_list_handler))
    .route("/id/{id}", get(get_pool_by_id_handler))
    .route("/", post(create_pool_handler))
    .route("/id/{id}", put(update_pool_by_id_handler))
    .route("/id/{id}", delete(delete_pool_by_id_handler));
}
