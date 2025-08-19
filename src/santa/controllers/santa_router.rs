use axum::{Router};
use serde::{Serialize, Deserialize};

use crate::{santa::controllers::{member_router, pool_router::pool_router}, AppState};


#[derive(Serialize, Deserialize)]
pub struct CreatePoolRequestData {
    pub name : String,
    pub description : String,
    pub account_id : String,
    pub min_price : u64,
    pub max_price : u64
}

pub fn santa_router(state : &AppState) -> Router<AppState> {
    return Router::new()
    .nest("/pools", pool_router(state))
    .nest("/members", member_router(state));
}
