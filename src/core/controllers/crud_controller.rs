use std::collections::HashMap;

use axum::{body::Body, extract::{Path, Request, State}, http::{HeaderMap, StatusCode}, response::IntoResponse, Json, Router};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ApiResponseStatus}, data_model::traits::ILocalObject, services::{IDbService, SQLiteDbService}}, AppState};

pub trait ICRUDController<T, O> where T : DeserializeOwned + Send + 'static, O : ILocalObject + Serialize + Clone {

    fn object_type_name() -> String;

    fn table_name() -> String;

    fn transform_func() -> fn(&SqliteRow) -> O;

    async fn get_objects_list_handler(State(state) : State<AppState>, headers : HeaderMap, request : Request<Body>) -> impl IntoResponse {
        let db_service = SQLiteDbService::new(&state);
        let objs_opt = db_service.get_all(Self::table_name().as_str(), Self::transform_func()).await;
        let objs = objs_opt.unwrap_or(vec![]);
        let resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(objs).unwrap());
        return (StatusCode::OK, Json(resp)).into_response();
    }

    async fn get_object_by_id_handler(State(state) : State<AppState>, Path(id) : Path<String>, headers : HeaderMap, request : Request<Body>) -> impl IntoResponse {
        let db_service = SQLiteDbService::new(&state);
        let obj_opt = db_service.get_one_by_prop(Self::table_name().as_str(), "id", id.as_str(), Self::transform_func()).await;
        let resp : ApiResponse;
        if obj_opt.is_none() { 
            let obj_type = Self::object_type_name();
            let err_msg = format!("Object (type \"{obj_type}\") with id \"{id}\" not found");
            resp = ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
            return (StatusCode::NOT_FOUND, Json(resp)).into_response(); 
        }
        let obj = obj_opt.unwrap();
        resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(obj).unwrap());
        return (StatusCode::OK, Json(resp)).into_response();
    }

    async fn create_object_handler(state : State<AppState>, Json(json) : Json<HashMap<String, serde_json::Value>>) -> impl IntoResponse {
        let json_value = serde_json::to_value(json);
        let resp : ApiResponse;
        if json_value.is_err() {
            let err_msg = format!("Request body is not json");
            resp = ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
            return (StatusCode::BAD_REQUEST, Json(resp)).into_response();
        }
        let obj_wrap : Result<T, serde_json::Error> = serde_json::from_value(json_value.unwrap());
        if obj_wrap.is_err() {
            let obj_type = Self::object_type_name();
            let err_msg = format!("Request body contains wrong json object (excepted \"{obj_type})\"");
            resp = ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
            return (StatusCode::BAD_REQUEST, Json(resp)).into_response();
        }
        let obj = obj_wrap.unwrap();
        resp = Self::create_object_and_return_id(obj, &state).await;
        if resp.is_ok() {
            return (StatusCode::OK, Json(resp)).into_response();
        } else {
            return (StatusCode::BAD_REQUEST, Json(resp)).into_response();
        }
    }

    async fn create_object_and_return_id(obj : T, state : &AppState) -> ApiResponse;

    async fn update_object_by_id_handler(State(state) : State<AppState>, Path(id) : Path<String>, headers : HeaderMap, Json(json) : Json<HashMap<String, serde_json::Value>>) -> impl IntoResponse {
        let db_service = SQLiteDbService::new(&state);
        let obj_exists = db_service.exists_by_prop(Self::table_name().as_str(), "id", id.as_str()).await;
        let resp : ApiResponse;
        if obj_exists.is_none_or(|b| {!b}) {
            let obj_type = Self::object_type_name(); 
            let err_msg = format!("Object (type \"{obj_type}\") with id \"{id}\" not found");
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
        
        let _ = db_service.update(Self::table_name().as_str(), "id", id.as_str(), props, values).await;

        let obj_opt = db_service.get_one_by_prop(Self::table_name().as_str(), "id", id.as_str(), Self::transform_func()).await;
        if obj_opt.is_none() { 
            let obj_type = Self::object_type_name(); 
            let err_msg = format!("Object (type \"{obj_type}\") with id \"{id}\" not found");
            resp = ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
            return (StatusCode::NOT_FOUND, Json(resp)).into_response(); 
        }
        let obj = obj_opt.unwrap();

        resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(obj).unwrap());
        return (StatusCode::OK, Json(resp)).into_response();
    }

    async fn delete_object_by_id_handler(State(state) : State<AppState>, Path(id) : Path<String>, headers : HeaderMap, request : Request<Body>) -> impl IntoResponse {
        let db_service = SQLiteDbService::new(&state);
        let _ = db_service.delete_one_by_prop(Self::table_name().as_str(), "id", id.as_str()).await;
        let obj_type = Self::object_type_name();
        let msg = format!("Object (type \"{obj_type}\") with id \"{id}\" successfully deleted");
        let resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(msg).unwrap());
        return (StatusCode::OK, Json(resp)).into_response();
    }

    fn objects_router(state : &AppState) -> Router<AppState>;
}