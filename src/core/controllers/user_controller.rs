use axum::{body::Body, extract::{Request, State}, routing::get, Router};

use crate::AppState;

async fn get_current_user_id(State(_) : State<AppState>, request : Request<Body>) -> String {
    let account_id = request.headers().get("account_id").unwrap().to_str().unwrap();
    return String::from(account_id);
}

pub fn user_controller(state: AppState) -> Router<AppState> {
    return Router::new()
    .route("/my_id", get(get_current_user_id));
}