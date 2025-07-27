use chrono::Utc;

use crate::{core::functions::new_id_safe, santa::{data_model::{enums::PoolState, traits::IPool}, services::{create_pool, is_pool_already_exists_by_id}}, AppState};

pub async fn user_create_pool(name : &str, description : &str, account_id : &str, min_price : u64, max_price : u64, is_creator_involved : bool, state : &AppState) -> () {
    let creation_date = Utc::now();
    let new_id = new_id_safe(is_pool_already_exists_by_id, state).await;
    create_pool(&new_id.as_str(), name, description, account_id, min_price, max_price, is_creator_involved, u64::MAX, creation_date, PoolState::Created, state).await;
}