use uuid::Uuid;

use crate::{core::{data_model::traits::{IAccount, IAccountSession, IClient, ILocalObject}, functions::{generate_random_token, validate_hash}}, AppState};

use super::{create_account_session, delete_account_session_by_account_id, delete_account_session_by_id, get_account_by_login, get_account_session_by_auth_token, get_account_session_by_id, get_account_session_by_refresh_token, get_client_by_client_name, is_account_session_already_exists_by_id, is_account_session_already_exists_by_token, update_account_session_last_usage_date_by_token, update_account_session_tokens_by_refresh_token};

pub async fn generate_tokens_unique_pair(state : &AppState) -> [String; 2] {
    let mut auth_token : String;
    loop {
        auth_token = generate_random_token();
        let is_account_session_already_exists =  is_account_session_already_exists_by_token(auth_token.as_str(), &state).await;
        if !is_account_session_already_exists { break; }
    }

    let mut refresh_token : String;
    loop {
        refresh_token = generate_random_token();
        let is_account_session_already_exists =  is_account_session_already_exists_by_token(refresh_token.as_str(), &state).await;
        if !is_account_session_already_exists && auth_token != refresh_token { break; }
    }

    return [auth_token, refresh_token];
}

async fn create_account_session_safe(account_id : &str, auth_token : &str, refresh_token : &str, state : &AppState) -> Option<impl IAccountSession> {
    
    let mut new_uuid : String;
    loop {
        new_uuid = Uuid::new_v4().to_string();
        let is_account_session_id_already_exists = is_account_session_already_exists_by_id(new_uuid.as_str(), &state).await;
        if !is_account_session_id_already_exists { break; }
    }
    create_account_session(&new_uuid, &account_id, &auth_token, &refresh_token, &state).await;
    return get_account_session_by_id(&new_uuid.as_str(), &state).await;
}

pub async fn is_client_valid(client_id : &str, client_secret : &str, state : &AppState) -> bool {
    let client = get_client_by_client_name(client_id, &state).await;
    if client.is_none() { return false; }
    let unwrap_client = client.unwrap();
    let is_client_password_valid = validate_hash(client_secret, unwrap_client.password_salt(), unwrap_client.password_hash());
    if !is_client_password_valid { return false; }
    return true;
}

pub async fn get_account_by_user_creditials(username : &str, password : &str, client_id : &str, client_secret : &str, state : &AppState) -> Option<impl IAccount> {
    let is_client_valid = is_client_valid(client_id, client_secret, &state).await;
    if !is_client_valid { return None; }
    let account = get_account_by_login(username, &state).await;
    if account.is_none() { return None; }
    let unwrap_account = account.unwrap();
    let is_account_valid = validate_hash(password, unwrap_account.passwrod_salt(), unwrap_account.password_hash());
    if !is_account_valid { return None; }
    return Some(unwrap_account);
}

pub async fn sign_in_by_user_creditials(username : &str, password : &str, client_id : &str, client_secret : &str, state : &AppState) -> Option<impl IAccountSession> {
    let account = get_account_by_user_creditials(username, password, client_id, client_secret, state).await;
    if account.is_none() { return None; }
    let [auth_token, refresh_token] = generate_tokens_unique_pair(&state).await;
    return create_account_session_safe(account.unwrap().id(), auth_token.as_str(), refresh_token.as_str(), &state).await;
}

pub async fn sign_in_by_refresh_token(refresh_token : &str, client_id : &str, client_secret : &str, state : &AppState) -> Option<impl IAccountSession> {
    let is_client_valid = is_client_valid(client_id, client_secret, &state).await;
    if !is_client_valid { return None; }
    let [new_auth_token, new_refresh_token] = generate_tokens_unique_pair(&state).await;
    update_account_session_tokens_by_refresh_token(refresh_token, new_auth_token.as_str(), new_refresh_token.as_str(), &state).await;
    return get_account_session_by_refresh_token(new_refresh_token.as_str(), &state).await;
}

pub async fn get_access_by_auth_token(auth_token : &str, client_id : &str, client_secret : &str, state : &AppState) -> Option<impl IAccountSession> {
    let is_client_valid = is_client_valid(client_id, client_secret, &state).await;
    if !is_client_valid { return None; }
    update_account_session_last_usage_date_by_token(auth_token, &state).await;
    return get_account_session_by_auth_token(auth_token, &state).await;
}

pub async fn sign_out(id : &str, state : &AppState) -> () {
    delete_account_session_by_id(id, &state).await;
}

pub async fn sign_out_from_all(account_id : &str, state : &AppState) -> () {
    delete_account_session_by_account_id(account_id, &state).await;
}