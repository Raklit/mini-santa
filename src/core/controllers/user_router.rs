use axum::{body::Body, extract::{Request, State}, http::StatusCode, response::IntoResponse, routing::get, Json, Router};

use crate::{core::{controllers::{ApiResponse, ApiResponseStatus}, data_model::traits::IPublicUserInfo, services::get_public_user_info_by_account_id}, AppState};

pub async fn get_current_user_id(State(_) : State<AppState>, request : Request<Body>) -> impl IntoResponse {
    let account_id = request.headers().get("account_id").unwrap().to_str().unwrap();
    let resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(account_id).unwrap());
    return (StatusCode::OK, Json(resp)).into_response();
}

pub async fn get_current_user_nickname(State(state) : State<AppState>, request : Request<Body>) -> impl IntoResponse {
    let account_id = request.headers().get("account_id").unwrap().to_str().unwrap();
    let public_user_info = get_public_user_info_by_account_id(account_id, &state).await.unwrap();
    let nickname = public_user_info.nickname();
    let resp = ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(nickname).unwrap());
    return (StatusCode::OK, Json(resp)).into_response();
}

pub fn user_router(_: &AppState) -> Router<AppState> {
    return Router::new()
    .route("/my_id", get(get_current_user_id))
    .route("/my_nickname", get(get_current_user_nickname));
}