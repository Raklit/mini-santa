use axum::{body::Body, extract::{Request, State}, routing::get, Router};

use crate::{core::{data_model::traits::IPublicUserInfo, services::get_public_user_info_by_account_id}, AppState};

pub async fn get_current_user_id(State(_) : State<AppState>, request : Request<Body>) -> String {
    let account_id = request.headers().get("account_id").unwrap().to_str().unwrap();
    return String::from(account_id);
}

pub async fn get_current_user_nickname(State(state) : State<AppState>, request : Request<Body>) -> String {
    let account_id = request.headers().get("account_id").unwrap().to_str().unwrap();
    let public_user_info = get_public_user_info_by_account_id(account_id, &state).await.unwrap();
    let nickname = public_user_info.nickname();
    return String::from(nickname);
}

pub fn user_router(state: AppState) -> Router<AppState> {
    return Router::new()
    .route("/my_id", get(get_current_user_id))
    .route("/my_nickname", get(get_current_user_nickname));
}