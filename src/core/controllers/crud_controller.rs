use std::collections::HashMap;

use axum::{body::Body, extract::{Path, Request, State}, http::{HeaderMap, HeaderValue, StatusCode}, response::IntoResponse, Json, Router};
use futures::executor;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ApiResponseStatus}, data_model::{implementations::RolesUserInfo, traits::{ILocalObject, IRolesUserInfo}}, services::{escape_string, row_to_role, row_to_roles_user_info, IDbService, SQLiteDbService}}, AppState};


#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
pub enum WhoIsExecutor {
    Nobody = 0,
    Other = 1,
    NoMatter = 2,
    Admin = 3,
    Moderator = 4,
    ResourceOwner = 5,
    PoolOwner = 6
}

pub trait ICRUDController<T, O> where T : DeserializeOwned + Send + 'static, O : ILocalObject + Serialize + Clone {

    fn object_type_name() -> String;

    fn table_name() -> String;

    fn transform_func() -> fn(&SqliteRow) -> O;

    async fn check_perm_create(state : &AppState, executor_id : &str) -> bool;

    async fn filter_many(state : &AppState, executor_id : &str) -> Option<Vec<O>>;

    async fn check_perm_get(state : &AppState, executor_id : &str, object_id : &str) -> bool {
        let objs = Self::filter_many(state, executor_id).await.unwrap_or(vec![]);
        return objs.iter().any(|o| { o.id() == object_id });
    }

    async fn check_perm_update(state : &AppState, executor_id : &str, object_id : &str) -> bool;

    async fn check_perm_delete(state : &AppState, executor_id : &str, object_id : &str) -> bool;

    async fn is_executor_admin(state : &AppState, executor_id : &str) -> bool {
        return Self::is_executor_has_role(state, executor_id, "administrator").await;
    }

    async fn is_executor_moderator(state : &AppState, executor_id : &str) -> bool {
        return Self::is_executor_has_role(state, executor_id, "moderator").await;
    }

    async fn is_executor_user(state : &AppState, executor_id : &str) -> bool {
        return Self::is_executor_has_role(state, executor_id, "user").await;
    }
    
    async fn is_executor_has_role(state : &AppState, executor_id : &str, role_name : &str) -> bool {
        let db_service = SQLiteDbService::new(state);
        
        let role_opt = db_service.get_one_by_prop("roles", "name", role_name, row_to_role).await;
        if role_opt.is_none() { return false; }
        let role = role_opt.unwrap();
        let role_id = role.id();

        let values = vec![executor_id];
        let roles_user_infos = db_service.get_many_by_prop("roles_user_infos", "account_id", values, row_to_roles_user_info).await.unwrap_or(Vec::new());

        return roles_user_infos.iter().any(|r| { r.role_id() == role_id });
    }

    async fn get_executor_id(headers : HeaderMap) -> String {
        return String::from(headers.get("account_id").unwrap().to_str().unwrap());
    }

    async fn only_for_admin_or_moderator(state : &AppState, executor_id : &str) -> (bool, WhoIsExecutor) {
        let is_admin = Self::is_executor_admin(state, executor_id).await;
        if is_admin {
            return (true, WhoIsExecutor::Admin);
        }

        let is_moderator = Self::is_executor_moderator(state, executor_id).await;
        if is_moderator {
            return (true, WhoIsExecutor::Moderator);
        }

        return (false, WhoIsExecutor::Other);
    }

    async fn basic_check_perm(state : &AppState, executor_id : &str) -> (Option<bool>, WhoIsExecutor) {
        let is_user = Self::is_executor_user(state, executor_id).await;
        if is_user {
            return (None, WhoIsExecutor::Other);
        }

        let (is_admin_or_moderator, role) = Self::only_for_admin_or_moderator(state, executor_id).await;
        if is_admin_or_moderator {
            return (Some(true), role);
        }

        return (Some(false), WhoIsExecutor::Nobody);
    }

    fn access_denied_response() -> impl IntoResponse {
        let err_msg = String::from("Access denied");
        let resp = ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
        return (StatusCode::FORBIDDEN, Json(resp)).into_response();

    }

    async fn get_objects_list_handler(State(state) : State<AppState>, headers : HeaderMap, request : Request<Body>) -> impl IntoResponse {
        let executor_id = Self::get_executor_id(headers).await;
        let objs_opt = Self::filter_many(&state, executor_id.as_str()).await;
        let objs = objs_opt.unwrap_or(vec![]);
        let resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(objs).unwrap());
        return (StatusCode::OK, Json(resp)).into_response();
    }

    async fn get_object_by_id_handler(State(state) : State<AppState>, Path(id) : Path<String>, headers : HeaderMap, request : Request<Body>) -> impl IntoResponse {
        let executor_id = Self::get_executor_id(headers).await;
        let object_id = escape_string(id.as_str());

        if !Self::check_perm_get(&state, executor_id.as_str(), object_id.as_str()).await {
            return Self::access_denied_response().into_response();
        }

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

    async fn create_object_handler(state : State<AppState>, headers : HeaderMap, Json(json) : Json<HashMap<String, serde_json::Value>>) -> impl IntoResponse {
        let executor_id = Self::get_executor_id(headers).await;
        let check_perm = Self::check_perm_create(&state, executor_id.as_str()).await;
        if !check_perm {
            return Self::access_denied_response().into_response();
        }
        
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
        let executor_id = Self::get_executor_id(headers).await;
        let object_id = escape_string(id.as_str());

        if !Self::check_perm_update(&state, executor_id.as_str(), object_id.as_str()).await {
            return Self::access_denied_response().into_response();
        }
        
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
        let executor_id = Self::get_executor_id(headers).await;
        let object_id = escape_string(id.as_str());

        if !Self::check_perm_delete(&state, executor_id.as_str(), object_id.as_str()).await {
            return Self::access_denied_response().into_response();
        }
        
        let db_service = SQLiteDbService::new(&state);
        let _ = db_service.delete_one_by_prop(Self::table_name().as_str(), "id", id.as_str()).await;
        let obj_type = Self::object_type_name();
        let msg = format!("Object (type \"{obj_type}\") with id \"{id}\" successfully deleted");
        let resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(msg).unwrap());
        return (StatusCode::OK, Json(resp)).into_response();
    }

    fn objects_router(state : &AppState) -> Router<AppState>;
}