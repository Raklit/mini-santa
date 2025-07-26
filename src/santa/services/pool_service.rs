use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqliteRow, Row};

use crate::{core::functions::{execute_script_template_wo_return, get_many_items_from_command, get_one_item_from_command, render_query_template}, santa::data_model::{enums::{PoolState, RoomState}, implementations::{Pool, Room}, traits::{IPool, IRoom}}, AppState};

fn row_to_pool(row : &SqliteRow) -> Pool {
    let id : &str = row.get("id");
    let name : &str = row.get("name");
    let description : &str = row.get("description");
    let account_id : &str = row.get("account_id");
    let min_price : u64 = row.get("min_price");
    let max_price : u64 = row.get("max_price");
    let is_creator_involved : bool = row.get("is_creator_involved");
    let lifetime : u64 = row.get("lifetime");
    let creation_date_str : &str = row.get("creation_date");
    let creation_date : DateTime<Utc> = DateTime::from_str(creation_date_str).unwrap();
    let pool_state_num : u8 = row.get("pool_state");
    let pool_state =  PoolState::try_from(usize::from(pool_state_num)).unwrap();
    return Pool::new(id, name, description, account_id, min_price, max_price, is_creator_involved, creation_date, lifetime, pool_state);
}

pub async fn get_pool_by_id(id : &str, state : &AppState) -> Option<impl IPool> {
    const GET_POOL_BY_ID_TEMPLATE : &str = "database_scripts/pool/get_pool_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(GET_POOL_BY_ID_TEMPLATE, &context, &state).await;
    return get_one_item_from_command(command.as_str(), state, row_to_pool).await;
}

pub async fn get_pools_by_account_id(account_id : &str, state : &AppState) -> Option<Vec<impl IPool>> {
    const GET_POOLS_BY_ACCOUNT_ID_TEMPLATE : &str = "database_scripts/pool/get_pools_by_account_id.sql";
    let mut context = tera::Context::new();
    context.insert("account_id", &account_id);
    
    let command = render_query_template(GET_POOLS_BY_ACCOUNT_ID_TEMPLATE, &context, &state).await;
    return get_many_items_from_command(command.as_str(), state, row_to_pool).await;
}

pub async fn set_pool_state(id : &str, pool_state : PoolState, state : &AppState) -> () {
    let pool_state_num = pool_state as usize;
    const SET_POOL_STATE_TEMPLATE : &str = "database_scripts/pool/set_pool_state.sql";
    let mut context = tera::Context::new();
    context.insert("id", id);
    context.insert("pool_state", &pool_state_num);
    
    execute_script_template_wo_return(SET_POOL_STATE_TEMPLATE, &context, state).await;
}

pub async fn create_pool(id : &str, name : &str, description : &str, account_id : &str, min_price : u64, max_price : u64, is_creator_involved : bool, lifetime : u64, creation_date : DateTime<Utc>, pool_state : PoolState,  state : &AppState) -> () {
    let pool_state_num = pool_state as usize;

    let mut context = tera::Context::new();
     context.insert("id", id);
     context.insert("name", name);
     context.insert("description", description);
     context.insert("account_id", account_id);
     context.insert("min_price", &min_price);
     context.insert("max_price", &max_price);
     context.insert("is_creator_involved", &is_creator_involved);
     context.insert("lifetime", &lifetime);
     context.insert("creation_date", &creation_date);
     context.insert("pool_state", &pool_state_num);

     const CREATE_POOL_TEMPLATE : &str = "database_scripts/pool/create_pool.sql";
     execute_script_template_wo_return(CREATE_POOL_TEMPLATE, &context, &state).await;
}

pub async fn delete_pool_by_id(id : &str, state : &AppState) -> () {
    let mut context = tera::Context::new();
     context.insert("id", id);
 
     const DELETE_POOL_TEMPLATE : &str = "database_scripts/pool/delete_pool_by_id.sql";
     execute_script_template_wo_return(DELETE_POOL_TEMPLATE, &context, &state).await;
}