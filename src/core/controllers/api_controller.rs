use axum::{body::Body, extract::State, http::{HeaderValue, Request, StatusCode}, middleware::{from_fn_with_state, Next}, response::IntoResponse, routing::{get, post}, Router};
use axum_auth::AuthBearer;
use chrono::{Duration, Utc};

use crate::{core::{controllers::{get_current_user_id, get_current_user_nickname, sign_in, sign_up}, data_model::traits::{IAccountRelated, IAccountSession}, services::get_access_by_access_token}, AppState};

// easy routes

async fn hello() -> String {
    return String::from("Hello World!");
}

async fn ping() -> String {
    return String::from("pong");
}

// auth middlware

async fn check_auth(State(state) : State<AppState>, AuthBearer(access_token) : AuthBearer, mut request : Request<Body>, next : Next) -> impl IntoResponse {
    let now_time = Utc::now();

    if access_token.is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "need auth token").into_response()); 
    }
    let account_session_option  = get_access_by_access_token(&access_token, &state).await;
    if account_session_option.is_none() {
        return Err((StatusCode::FORBIDDEN, "token doesn't exists").into_response()); 
    }
    let account_session = account_session_option.unwrap();
    let access_token_lifetime = state.config.lock().await.auth.access_token_lifetime;
    
    let lifetime_end = account_session.access_token_creation_date() + Duration::seconds(access_token_lifetime.try_into().unwrap());
    if lifetime_end < now_time {
        return Err((StatusCode::UNAUTHORIZED, "access token is expired").into_response());
    }

    while request.headers_mut().contains_key("account_id") {
        request.headers_mut().remove("account_id");
    }
    request.headers_mut().append("account_id", HeaderValue::from_str(account_session.account_id()).unwrap());
    return Ok(next.run(request).await);

}


// complex routers

pub fn auth_router() -> Router<AppState> {
    return Router::new()
    .route("/token", get(sign_in))
}

pub fn user_controller(state: AppState) -> Router<AppState> {
    return Router::new()
    .route("/my_id", get(get_current_user_id))
    .route("/my_nickname", get(get_current_user_nickname));
}


// routers groups

pub fn no_auth_api_router() -> Router<AppState> {
    return Router::new()
        .route("/hello", get(hello))
        .route("/ping", get(ping))
        .route("/sign_up", post(sign_up))
}

pub fn need_auth_api_router(state : AppState) -> Router<AppState> {
    return Router::new()
        .nest("/user", user_controller(state.clone()))
        .layer(from_fn_with_state(state.clone(), check_auth))
}

pub fn api_router(state : AppState) -> Router<AppState> {
    return need_auth_api_router(state).merge(no_auth_api_router());
}