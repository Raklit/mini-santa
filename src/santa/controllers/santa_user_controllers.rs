use axum::{body::Body, extract::State, http::{HeaderMap, Request, StatusCode}, response::IntoResponse, Form, Json};
use serde::{Deserialize, Serialize};

use crate::{santa::services::user_create_pool, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreatePoolRequestData {
    pub name : String,
    pub description : String,
    pub account_id : String,
    pub min_price : u64,
    pub max_price : u64
}

pub async fn create_pc(State(state) : State<AppState>, Json(json) : Json<CreatePoolRequestData>) -> String {
    user_create_pool(json.name.as_str(), json.description.as_str(), json.account_id.as_str(), json.min_price, json.max_price.clone(), &state).await;
    return String::new();
}