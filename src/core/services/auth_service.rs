use std::time::Duration;

use chrono::Utc;
use regex::Regex;

use crate::{core::{data_model::traits::{IAccount, IAccountRelated, IAccountSession, IAuthCode, IClient, ILocalObject}, functions::{generate_id, generate_random_token, validate_hash}, services::{create_account, create_public_user_info, create_recovery_user_info, delete_auth_code_by_id, get_auth_code_by_code, is_account_already_exists_by_id, is_account_already_exists_by_login, is_public_user_info_already_exists_by_id, is_public_user_info_already_exists_by_nickname, is_recovery_user_info_already_exists_by_email, is_recovery_user_info_already_exists_by_id}}, AppState};

use super::{create_account_session, delete_account_session_by_account_id, delete_account_session_by_id, get_account_by_login, get_account_session_by_access_token, get_account_session_by_id, get_account_session_by_refresh_token, get_client_by_client_name, is_account_session_already_exists_by_id, is_account_session_already_exists_by_token, update_account_session_last_usage_date_by_token, update_account_session_tokens_by_refresh_token};

pub async fn generate_tokens_unique_pair(state : &AppState) -> [String; 2] {
    let mut access_token : String;
    loop {
        access_token = generate_random_token();
        let is_account_session_already_exists =  is_account_session_already_exists_by_token(access_token.as_str(), &state).await;
        if !is_account_session_already_exists { break; }
    }

    let mut refresh_token : String;
    loop {
        refresh_token = generate_random_token();
        let is_account_session_already_exists =  is_account_session_already_exists_by_token(refresh_token.as_str(), &state).await;
        if !is_account_session_already_exists && access_token != refresh_token { break; }
    }

    return [access_token, refresh_token];
}

async fn create_account_session_safe(account_id : &str, access_token : &str, refresh_token : &str, state : &AppState) -> Option<impl IAccountSession> {
    
    let mut new_id : String;
    loop {
        new_id = generate_id().await;
        let is_account_session_id_already_exists = is_account_session_already_exists_by_id(new_id.as_str(), &state).await;
        if !is_account_session_id_already_exists { break; }
    }
    create_account_session(&new_id, &account_id, &access_token, &refresh_token, &state).await;
    return get_account_session_by_id(&new_id.as_str(), &state).await;
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
    let [access_token, refresh_token] = generate_tokens_unique_pair(&state).await;
    if account.is_none() { return None; }
    return create_account_session_safe(account.unwrap().id(), access_token.as_str(), refresh_token.as_str(), &state).await;
}

pub async fn sign_in_by_refresh_token(refresh_token : &str, client_id : &str, client_secret : &str, state : &AppState) -> Option<impl IAccountSession> {
    let now_time = Utc::now();
    let is_client_valid = is_client_valid(client_id, client_secret, &state).await;
    if !is_client_valid { return None; }
    
    let [new_access_token, new_refresh_token] = generate_tokens_unique_pair(&state).await;
    update_account_session_tokens_by_refresh_token(refresh_token, new_access_token.as_str(), new_refresh_token.as_str(), &state).await;
    
    let session_option = get_account_session_by_refresh_token(new_refresh_token.as_str(), &state).await;
    if session_option.is_none() { return None; }
    let session = session_option.unwrap();
    
    let refresh_token_lifetime = state.config.lock().await.auth.refresh_token_lifetime;
    let lifetime_end = session.refresh_token_creation_date() + Duration::from_secs(refresh_token_lifetime);
    if lifetime_end < now_time {
        delete_account_session_by_id(session.id(), state).await;
        return None;
    }

    return Some(session);
}

pub async fn sign_in_by_auth_code(auth_code : &str, client_id : &str, client_secret : &str, state : &AppState) -> Option<impl IAccountSession> {
    let now_time = Utc::now();
    let is_client_valid = is_client_valid(client_id, client_secret, &state).await;
    if !is_client_valid { return None; }
    
   let auth_code_option = get_auth_code_by_code(auth_code, state).await;
   if auth_code_option.is_none() { return None; }
   let auth_code = auth_code_option.unwrap();
   
   let auth_code_lifetime = state.config.lock().await.auth.auth_code_lifetime;
   let lifetime_end = auth_code.creation_date() + Duration::from_secs(auth_code_lifetime);
   if lifetime_end < now_time {
        delete_auth_code_by_id(auth_code.id(), state).await;
        return None;
   }

   let [access_token, refresh_token] = generate_tokens_unique_pair(&state).await;
   let session_option = create_account_session_safe(auth_code.account_id(), access_token.as_str(), refresh_token.as_str(), &state).await;
   if session_option.is_none() { return None; }
   delete_auth_code_by_id(auth_code.id(), state).await;
   
   return session_option;
}

pub async fn get_access_by_access_token(access_token : &str, state : &AppState) -> Option<impl IAccountSession> {
    let now_time = Utc::now();
    let account_session_option = get_account_session_by_access_token(access_token, &state).await;
    if account_session_option.is_none() { return None; }
    let account_session = account_session_option.unwrap();
    let access_token_lifetime = state.config.lock().await.auth.access_token_lifetime;
    let lifetime_end = account_session.access_token_creation_date() + Duration::from_secs(access_token_lifetime);
    if lifetime_end < now_time {
        delete_account_session_by_id(account_session.id(), state).await;
        return None;
    }
    update_account_session_last_usage_date_by_token(access_token, &state).await;
    return Some(account_session);

}

pub async fn sign_out(id : &str, state : &AppState) -> () {
    delete_account_session_by_id(id, &state).await;
}

pub async fn sign_out_from_all(account_id : &str, state : &AppState) -> () {
    delete_account_session_by_account_id(account_id, &state).await;
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SignUpStatus {
    OK = 0,

    // login
    LoginContainsNotAllowedChars = 1,
    LoginExists = 3,
    LoginIsEmpty = 4,
    LoginIsLong = 5,

    // password
    PasswordContainsNotAllowedChars = 6,
    PasswordDoesNotMatch = 7,
    PasswordIsEmpty = 8,
    PasswordIsShort = 9,
    PasswordIsLong = 10,
    PasswordIsCommon = 11,
    
    // email
    EmailIsInvalid = 12,
    EmailIsEmpty = 13,
    EmailAlreadyInUse = 14,

    // nickname
    NicknameContainsNotAllowedChars = 15,
    NicknameExists = 16,
    NicknameIsEmpty = 17,
    NicknameIsLong = 18,
    NicknameIsRestricted = 19
}

fn is_login_chars_valid(login : &str) -> bool {
    const LOGIN_ALLOWED_CHARACTERS : &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    return login.chars().all( | c : char | -> bool { LOGIN_ALLOWED_CHARACTERS.contains(c) } );
}

async fn is_login_valid(login : &str, state : &AppState) -> SignUpStatus {
    const LOGIN_MAX_LENGTH : usize = 128;
    
    if login.is_empty() { return SignUpStatus::LoginIsEmpty; }
    if login.len() > LOGIN_MAX_LENGTH { return SignUpStatus::LoginIsLong; }

    let login_chars_valid = is_login_chars_valid(login);
    if !login_chars_valid { return SignUpStatus::LoginContainsNotAllowedChars; }

    let login_exists = is_account_already_exists_by_login(login, state).await;
    if login_exists { return SignUpStatus::LoginExists; }

    return SignUpStatus::OK;
}

fn is_password_chars_valid(password : &str) -> bool {
    const PASSWORD_ALLOWED_CHARACTERS : &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    return password.chars().all( | c : char | -> bool { PASSWORD_ALLOWED_CHARACTERS.contains(c) } );
}
/// TODO: NOT IMPLEMENTET YET
async fn is_password_common(password : &str, state : &AppState) -> bool {
    return false;
}

async fn is_password_valid(password : &str, state : &AppState) -> SignUpStatus {
    const PASSWORD_MIN_LENGTH : usize = 12;
    const PASSWORD_MAX_LENGTH : usize = 128;
    
    if password.is_empty() { return SignUpStatus::PasswordIsEmpty; }
    if password.len() < PASSWORD_MIN_LENGTH { return SignUpStatus::PasswordIsShort; }
    if password.len() > PASSWORD_MAX_LENGTH { return SignUpStatus::PasswordIsLong; }

    let password_chars_valid = is_password_chars_valid(password);
    if !password_chars_valid { return SignUpStatus::PasswordContainsNotAllowedChars; }

    let password_common = is_password_common(password, state).await;
    if password_common { return SignUpStatus::PasswordIsCommon; }

    return SignUpStatus::OK;
}

fn is_nickname_chars_valid(nickname : &str) -> bool {
    const NICKNAME_ALLOWED_CHARACTERS : &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    return nickname.chars().all( | c : char | -> bool { NICKNAME_ALLOWED_CHARACTERS.contains(c) } );
}

/// TODO: NOT IMPLEMENTET YET
async fn is_nickname_restricted(nickname : &str, state : &AppState) -> bool {
    return false;
}

async fn is_nickname_valid(nickname : &str, state : &AppState) -> SignUpStatus {
    const NICKNAME_MAX_LENGTH : usize = 128;

    if nickname.is_empty() { return SignUpStatus::NicknameIsEmpty; }
    if nickname.len() > NICKNAME_MAX_LENGTH { return SignUpStatus::NicknameIsLong; }
    
    let nickname_chars_valid = is_nickname_chars_valid(nickname);
    if !nickname_chars_valid { return SignUpStatus::NicknameContainsNotAllowedChars; }

    let nickname_restricted = is_nickname_restricted(nickname, state).await;
    if nickname_restricted { return SignUpStatus::NicknameIsRestricted; }

    let nickname_exists = is_public_user_info_already_exists_by_nickname(nickname, state).await;
    if nickname_exists { return SignUpStatus::NicknameExists; }

    return SignUpStatus::OK;
}

async fn is_email_valid(email : &str, state : &AppState) -> SignUpStatus {
    if email.is_empty() { return SignUpStatus::EmailIsEmpty; }

    // not RFC822, but it is hard because r"" instead of r''
    const EMAIL_PATTERN : &str = r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$";
    let re = Regex::new(EMAIL_PATTERN).unwrap();
    let email_valid = re.is_match(email);
    if !email_valid { return SignUpStatus::EmailIsInvalid; }
    
    let email_exists = is_recovery_user_info_already_exists_by_email(email, state).await;
    if email_exists { return SignUpStatus::EmailAlreadyInUse; }
    
    return SignUpStatus::OK;
}

/// TODO: NOT IMPLEMENTET YET
async fn create_user(login : &str, password : &str, nickname : &str, email : &str, state : &AppState) -> () {
    let mut account_id : String;
    loop {
        account_id = generate_id().await;
        let account_exists = is_account_already_exists_by_id(account_id.as_str(), state).await;
        if !account_exists { break; }
    }

    let mut public_user_info_id : String;
    loop {
        public_user_info_id = generate_id().await;
        let public_user_info_exists = is_public_user_info_already_exists_by_id(public_user_info_id.as_str(), state).await;
        if !public_user_info_exists { break; }
    }

    let mut recovery_user_info_id : String;
    loop {
        recovery_user_info_id = generate_id().await;
        let recovery_user_info_exists = is_recovery_user_info_already_exists_by_id(recovery_user_info_id.as_str(), state).await;
        if !recovery_user_info_exists { break; }
    }


    create_account(account_id.as_str(), login, password, state).await;
    create_public_user_info(public_user_info_id.as_str(), account_id.as_str(), nickname, "", state).await;
    create_recovery_user_info(recovery_user_info_id.as_str(), account_id.as_str(), email, "", state).await;
}

pub async fn user_sign_up(login : &str, password : &str, confirm_password : &str, nickname : &str, email : &str, state : &AppState) -> Vec<SignUpStatus> {
    let mut result : Vec<SignUpStatus> = Vec::new();

    if password != confirm_password {
        result.push(SignUpStatus::PasswordDoesNotMatch);
    }
    result.push(is_login_valid(login, state).await);
    result.push(is_password_valid(password, state).await);
    result.push(is_nickname_valid(nickname, state).await);
    result.push(is_email_valid(email, state).await);

    let data_valid = result.clone().into_iter().all(|s : SignUpStatus| -> bool { s == SignUpStatus::OK });
    if data_valid {
        create_user(login, password, nickname, email, state).await;
    }
    return result;
}