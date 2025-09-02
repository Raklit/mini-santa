use axum::{routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;

use crate::{core::{controllers::{ApiResponse, ICRUDController, WhoIsExecutor}, data_model::traits::IAccountRelated, services::{IDbService, SQLiteDbService}}, santa::{data_model::{enums::PoolState, implementations::{Member, Pool}, traits::{IPool, IPoolRelated}}, services::{row_to_member, row_to_pool, user_add_member_to_pool}}, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateMemberRequestData {
    pub account_id : String,
    pub pool_id : String,
    pub wishlist : Option<String>
}

pub struct MemberCRUDController {}

impl MemberCRUDController {
    async fn basic_check_owner(state : &AppState, executor_id : &str, object_id : &str) -> (Option<bool>, WhoIsExecutor) {
        let (basic_check, role) = Self::basic_check_perm(state, executor_id).await;
        if basic_check.is_some() { return (basic_check, role); }
        
        let db_service = SQLiteDbService::new(state);

        let member_opt = db_service.get_one_by_prop(Self::table_name().as_str(), "id", object_id, Self::transform_func()).await;
        if member_opt.is_none() { return (Some(true), WhoIsExecutor::NoMatter); }
        let member = member_opt.unwrap();

        let is_resource_owner = member.account_id() == executor_id;
        if is_resource_owner { return (None, WhoIsExecutor::ResourceOwner); }

        let pool_id = member.pool_id();
        let pool_opt = db_service.get_one_by_prop("pools", "id", pool_id, row_to_pool).await;
        if pool_opt.is_none() { return (Some(false), WhoIsExecutor::NoMatter); }
        let pool = pool_opt.unwrap();

        let is_pool_owner = pool.account_id() == executor_id;
        if is_pool_owner { return (None, WhoIsExecutor::PoolOwner); }

        return (None, WhoIsExecutor::Other);
    }

    async fn get_pool_by_member_id(state : &AppState, member_id : &str) -> Option<Pool> {
        let db_service = SQLiteDbService::new(state);
        
        let member_opt = db_service.get_one_by_prop("members", "id", member_id, row_to_member).await;
        if member_opt.is_none() { return None; }
        let member = member_opt.unwrap();
        
        let pool_opt = db_service.get_one_by_prop("pools", "id", member.pool_id(), row_to_pool).await;
        if pool_opt.is_none() { return None; }
        let pool = pool_opt.unwrap();

        return Some(pool);
    }
}

impl ICRUDController<CreateMemberRequestData, Member> for MemberCRUDController {
    fn object_type_name() -> String { return String::from("member"); }

    fn table_name() -> String { return String::from("members"); }

    fn transform_func() -> fn(&SqliteRow) -> Member { return row_to_member; }

    async fn create_object_and_return_id(executor_id : &str, obj : CreateMemberRequestData, state : &AppState) -> ApiResponse {
        let (basic_check, role) = Self::basic_check_perm(state, executor_id).await;
        if basic_check.is_some_and(|b| {!b}) { 
           return Self::acting_like_another_user_api_response();
        }
        if role == WhoIsExecutor::Other {
            return Self::acting_like_another_user_api_response();
        }
        if obj.account_id.as_str() != executor_id {
            return Self::acting_like_another_user_api_response();
        }
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

        let (basic_check, _) = Self::basic_check_perm(state, executor_id).await;
        if basic_check.is_some_and(|b| {b}) { return db_service.get_all(Self::table_name().as_str(), Self::transform_func()).await; }

        let is_user = Self::is_executor_user(state, executor_id).await;
        if is_user {
            let values = vec![executor_id];
            return db_service.get_many_by_prop(Self::table_name().as_str(), "account_id", values, Self::transform_func()).await;
        }

        return None;
    }
    
    async fn check_perm_update(state : &AppState, executor_id : &str, object_id : &str) -> bool {
        let (basic_check_owner, role) = Self::basic_check_owner(state, executor_id, object_id).await;
        if basic_check_owner.is_some() { return basic_check_owner.unwrap(); }
        if role == WhoIsExecutor::Other { return false; }
        
        let pool_opt = Self::get_pool_by_member_id(state, object_id).await;
        if pool_opt.is_none() { return false; }
        let pool = pool_opt.unwrap();
        
        if (role == WhoIsExecutor::PoolOwner && pool.state() == PoolState::Pooling) || (role == WhoIsExecutor::ResourceOwner && pool.state() == PoolState::Open) { 
            return true;
        }

        return false;
    }
    
    async fn check_perm_delete(state : &AppState, executor_id : &str, object_id : &str) -> bool {
        let (basic_check_owner, role) = Self::basic_check_owner(state, executor_id, object_id).await;
        if basic_check_owner.is_some() { return basic_check_owner.unwrap(); }
        if role == WhoIsExecutor::Other { return false; }
        
        let pool_opt = Self::get_pool_by_member_id(state, object_id).await;
        if pool_opt.is_none() { return false; }
        let pool = pool_opt.unwrap();
        
        if (role == WhoIsExecutor::PoolOwner && pool.state() == PoolState::Ended) || (role == WhoIsExecutor::ResourceOwner && pool.state() == PoolState::Open) { 
            return true;
        }

        return false;
    }
}

pub fn member_router(state : &AppState) -> Router<AppState> {
    return MemberCRUDController::objects_router(state);
}