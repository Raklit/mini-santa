use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqliteRow, Row};

use crate::{core::services::{IDbService, SQLiteDbService}, santa::data_model::{enums::PoolState, implementations::Pool, traits::IPool}, AppState};

pub fn row_to_pool(row : &SqliteRow) -> Pool {
    let id : &str = row.get("id");
    let name : &str = row.get("name");
    let description : &str = row.get("description");
    let account_id : &str = row.get("account_id");
    let min_price : u64 = row.get("min_price");
    let max_price : u64 = row.get("max_price");
    let lifetime : u64 = row.get("lifetime");
    let creation_date_str : &str = row.get("creation_date");
    let creation_date : DateTime<Utc> = DateTime::from_str(creation_date_str).unwrap();
    let pool_state_num : u8 = row.get("pool_state");
    let pool_state =  PoolState::try_from(usize::from(pool_state_num)).unwrap();
    return Pool::new(id, name, description, account_id, min_price, max_price, creation_date, lifetime, pool_state);
}

pub async fn get_pool_by_id(id : &str, state : &AppState) -> Option<impl IPool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("pools", "id", id, row_to_pool).await;
}

pub async fn get_pools_by_account_id(account_id : &str, state : &AppState) -> Option<Vec<impl IPool>> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_many_by_prop("pools", "account_id", vec![account_id], row_to_pool).await;
}

pub async fn set_pool_state(id : &str, pool_state : PoolState, state : &AppState) -> () {
    let pool_state_num = pool_state as usize;
    let pool_state_string = pool_state_num.to_string();
    let pool_state_str = pool_state_string.as_str();

    let db_service = SQLiteDbService::new(state);
    db_service.update("pools", "id", id, vec!["pool_state"], vec![pool_state_str]).await;
}

pub async fn is_pool_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("pools", "id", id).await;
}

pub async fn create_pool(id : &str, name : &str, description : &str, account_id : &str, min_price : u64, max_price : u64, lifetime : u64, creation_date : DateTime<Utc>, pool_state : PoolState,  state : &AppState) -> () {
    let pool_state_num = pool_state as usize;
    
    let pool_state_string = pool_state_num.to_string();
    let pool_state_str = pool_state_string.as_str();
    let creation_date_string = creation_date.to_rfc3339();
    let creation_date_str = creation_date_string.as_str();
    let min_price_string = min_price.to_string();
    let min_price_str = min_price_string.as_str();
    let max_price_string = max_price.to_string();
    let max_price_str = max_price_string.as_str();
    let lifetime_string = lifetime.to_string();
    let lifetime_str = lifetime_string.as_str();

    let db_service = SQLiteDbService::new(state);
    let _ = db_service.insert("pools", 
    vec!["id", "name", "description", "account_id", "min_price", "max_price", "lifetime", "creation_date", "pool_state"],
    vec![vec![id, name, description, account_id, min_price_str, max_price_str, lifetime_str, creation_date_str, pool_state_str]]).await;
}

pub async fn delete_pool_by_id(id : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    db_service.delete_one_by_prop("pools", "id", id).await;
}