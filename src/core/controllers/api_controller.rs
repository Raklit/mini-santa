use axum::{body::Body, extract::State, http::{HeaderValue, Request, StatusCode}, middleware::{from_fn_with_state, Next}, response::IntoResponse, routing::get, Router};
use axum_auth::AuthBearer;

use crate::{core::{controllers::user_controller, data_model::traits::IAccountRelated, services::{get_access_by_auth_token, update_account_session_last_usage_date_by_token}}, AppState};

async fn hello() -> String {
    return String::from("Hello World!");
}

async fn ping() -> String {
    return String::from("pong");
}

async fn check_auth(State(state) : State<AppState>, AuthBearer(access_token) : AuthBearer, mut request : Request<Body>, next : Next) -> impl IntoResponse {

    if access_token.is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "need auth token").into_response()); 
    }
    let account_session_option  = get_access_by_auth_token(&access_token, &state).await;
    if account_session_option.is_none() {
        return Err((StatusCode::FORBIDDEN, "token doesn't exists").into_response()); 
    }
    while request.headers_mut().contains_key("account_id") {
        request.headers_mut().remove("account_id");
    }
    let account_session = account_session_option.unwrap();
    update_account_session_last_usage_date_by_token(access_token.as_str(), &state).await;
    request.headers_mut().append("account_id", HeaderValue::from_str(account_session.account_id()).unwrap());
    return Ok(next.run(request).await);

}

pub fn no_auth_api_router() -> Router<AppState> {
    return Router::new()
        .route("/hello", get(hello))
        .route("/ping", get(ping))
}

pub fn need_auth_api_router(state : AppState) -> Router<AppState> {
    return Router::new()
        .nest("/user", user_controller(state.clone()))
        .layer(from_fn_with_state(state.clone(), check_auth))
}

pub fn api_router(state : AppState) -> Router<AppState> {
    return need_auth_api_router(state).merge(no_auth_api_router());
}