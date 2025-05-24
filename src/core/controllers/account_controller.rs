use std::collections::HashMap;

use axum::extract::{Query, State};

use crate::{core::{data_model::traits::{IAccountRelated, ILocalObject}, services::{get_account_by_id, sign_in_by_user_creditials}}, AppState};

pub async fn sign_in(params: Query<HashMap<String, String>>, State(state) : State<AppState>) -> String {
    let grant_type = params.get("grant_type").map_or("", |v| v);
    let client_id = params.get("client_id").map_or("", |v| v);
    let client_secret =params.get("client_secret").map_or("", |v| v);
    if grant_type == "password" {
        let username = params.get("username").map_or("", |v| v);
        let password = params.get("password").map_or("", |v| v);
        let account_session = sign_in_by_user_creditials(username, password, client_id, client_secret, &state).await;
        if account_session.is_some() {
            let account = get_account_by_id(account_session.unwrap().account_id(), &state).await; 
            return String::from(account.unwrap().id());
        } else {
            return String::from("NONE");
        }
    }
    return String::new();
}