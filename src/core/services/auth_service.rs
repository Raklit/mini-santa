use std::{collections::HashMap, time::Duration};

use chrono::Utc;
use regex::Regex;

use crate::{core::{controllers::{ApiResponse, ApiResponseStatus}, data_model::traits::{IAccount, IAccountRelated, IAccountSession, IAuthCode, IClient, IInvite, ILocalObject, IPublicUserInfo, IRecoveryUserInfo}, functions::{generate_random_token, validate_hash}, services::{create_account, create_public_user_info, create_random_invite_code_safe, create_recovery_user_info, create_roles_user_info, db_service, delete_auth_code_by_id, get_account_by_id, get_auth_code_by_code, get_public_user_info_by_account_id, get_recovery_user_info_by_account_id, is_account_already_exists_by_login, is_public_user_info_already_exists_by_nickname, is_recovery_user_info_already_exists_by_email, row_to_account, row_to_invite, row_to_role, row_to_roles_user_info, set_account_login, set_account_password, set_email, set_nickname, IDbService, SQLiteDbService}}, AppState};

use super::{create_account_session, delete_account_sessions_by_account_id, delete_account_session_by_id, get_account_by_login, get_account_session_by_access_token, get_account_session_by_id, get_account_session_by_refresh_token, get_client_by_client_name, is_account_session_already_exists_by_token, update_account_session_last_usage_date_by_token, update_account_session_tokens_by_refresh_token};

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
    let db_service = SQLiteDbService::new(state);
    let new_id_option = db_service.new_id("account_sessions").await;
    if new_id_option.is_none() { return None; }
    let new_id = new_id_option.unwrap();
    create_account_session(&new_id, &account_id, &access_token, &refresh_token, &state).await;
    return get_account_session_by_id(&new_id.as_str(), &state).await;
}

pub async fn is_client_valid(client_id : Option<String>, client_secret : Option<String>, state : &AppState) -> bool {
    if client_id.is_none() { return true; }
    let client_id_unwrap = client_id.unwrap();
    let client_id_unwrap_str = client_id_unwrap.as_str();
    let client_option = get_client_by_client_name(client_id_unwrap_str, &state).await;
    if client_option.is_none() { return false; }
    let client = client_option.unwrap();
    if client.no_pwd() { return true; }

    if client_secret.is_none() { return false; }
    let client_secret_unwrap = client_secret.unwrap();
    let client_secret_unwrap_str = client_secret_unwrap.as_str();
    let is_client_password_valid = validate_hash(client_secret_unwrap_str, client.password_salt(), client.password_hash());
    if !is_client_password_valid { return false; }
    return true;
}

pub async fn get_account_by_user_creditials(username : &str, password : &str, client_id : Option<String>, client_secret : Option<String>, state : &AppState) -> Option<impl IAccount> {
    let is_client_valid = is_client_valid(client_id, client_secret, &state).await;
    if !is_client_valid { return None; }
    let account = get_account_by_login(username, &state).await;
    if account.is_none() { return None; }
    let unwrap_account = account.unwrap();
    let is_account_valid = validate_hash(password, unwrap_account.passwrod_salt(), unwrap_account.password_hash());
    if !is_account_valid { return None; }
    return Some(unwrap_account);
}

pub async fn sign_in_by_user_creditials(username : &str, password : &str, client_id : Option<String>, client_secret : Option<String>, state : &AppState) -> Option<impl IAccountSession> {
    let account = get_account_by_user_creditials(username, password, client_id, client_secret, state).await;
    let [access_token, refresh_token] = generate_tokens_unique_pair(&state).await;
    if account.is_none() { return None; }
    return create_account_session_safe(account.unwrap().id(), access_token.as_str(), refresh_token.as_str(), &state).await;
}

pub async fn sign_in_by_refresh_token(refresh_token : &str, client_id: Option<String>, client_secret : Option<String>, state : &AppState) -> Option<impl IAccountSession> {
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

pub async fn sign_in_by_auth_code(auth_code : &str, client_id : Option<String>, client_secret : Option<String>, state : &AppState) -> Option<impl IAccountSession> {
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
    delete_account_sessions_by_account_id(account_id, &state).await;
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum SignUpStatus {
    OK = 0,

    // login
    LoginContainsNotAllowedChars = 1,
    LoginExists = 2,
    LoginIsEmpty = 3,
    LoginIsLong = 4,

    // password
    PasswordContainsNotAllowedChars = 5,
    PasswordDoesNotMatch = 6,
    PasswordIsEmpty = 7,
    PasswordIsShort = 8,
    PasswordIsLong = 9,
    PasswordIsCommon = 10,
    
    // email
    EmailIsInvalid = 11,
    EmailIsEmpty = 12,
    EmailAlreadyInUse = 13,

    // nickname
    NicknameContainsNotAllowedChars = 14,
    NicknameExists = 15,
    NicknameIsEmpty = 16,
    NicknameIsLong = 17,
    NicknameIsRestricted = 18,

    // invite code
    InviteCodeIsEmpty = 19,
    InviteCodeDoesNotExists = 20,

    // other
    DBConnectionLost = 21,
}

pub fn sign_up_error_description_map() -> HashMap<SignUpStatus, String> {
    let mut result = HashMap::<SignUpStatus, String>::new();
    result.insert(SignUpStatus::OK, String::from("Success"));

    result.insert(SignUpStatus::LoginContainsNotAllowedChars, String::from("Login contains not allowed characters"));
    result.insert(SignUpStatus::LoginExists, String::from("Login is already in use by another account"));
    result.insert(SignUpStatus::LoginIsEmpty, String::from("Login is empty"));
    result.insert(SignUpStatus::LoginIsLong, String::from("Login is too long"));
    
    result.insert(SignUpStatus::PasswordContainsNotAllowedChars, String::from("Password contains not allowed characters"));
    result.insert(SignUpStatus::PasswordDoesNotMatch, String::from("Passwords do not match"));
    result.insert(SignUpStatus::PasswordIsEmpty, String::from("Password is empty"));
    result.insert(SignUpStatus::PasswordIsShort, String::from("Password is too short"));
    result.insert(SignUpStatus::PasswordIsLong, String::from("Password is too long"));
    result.insert(SignUpStatus::PasswordIsCommon, String::from("Password is too common"));
    
    result.insert(SignUpStatus::EmailIsInvalid, String::from("Email is invalid"));
    result.insert(SignUpStatus::EmailIsEmpty, String::from("Email is empty"));
    result.insert(SignUpStatus::EmailAlreadyInUse, String::from("Email is already in use by another account"));

    result.insert(SignUpStatus::NicknameContainsNotAllowedChars, String::from("Nickname contains not allowed characters"));
    result.insert(SignUpStatus::NicknameExists, String::from("Nickname is already in use by another account"));
    result.insert(SignUpStatus::NicknameIsEmpty, String::from("Nickname is empty"));
    result.insert(SignUpStatus::NicknameIsLong, String::from("Nickname is too long"));
    result.insert(SignUpStatus::NicknameIsRestricted, String::from("Nickname is restricted"));

    result.insert(SignUpStatus::InviteCodeIsEmpty, String::from("Invite code is empty"));
    result.insert(SignUpStatus::InviteCodeDoesNotExists, String::from("Invite code does not exist"));

    result.insert(SignUpStatus::DBConnectionLost, String::from("The connection to the database was lost"));

    return result;
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

    let login_exists_opt = is_account_already_exists_by_login(login, state).await;
    if login_exists_opt.is_none() { return SignUpStatus::DBConnectionLost; }
    if login_exists_opt.unwrap() { return SignUpStatus::LoginExists; }

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
    const NICKNAME_ALLOWED_CHARACTERS : &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789АБВГДЕЁЖЗИИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдеёжзиийклмнопрстуфхцчшщъыьэюя-_ ";
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

    let nickname_exists_opt = is_public_user_info_already_exists_by_nickname(nickname, state).await;
    if nickname_exists_opt.is_none() { return SignUpStatus::DBConnectionLost; }
    if nickname_exists_opt.unwrap() { return SignUpStatus::NicknameExists; }

    return SignUpStatus::OK;
}

async fn is_email_valid(email : &str, state : &AppState) -> SignUpStatus {
    if email.is_empty() { return SignUpStatus::EmailIsEmpty; }

    // not RFC822, but it is hard because r"" instead of r''
    const EMAIL_PATTERN : &str = r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$";
    let re = Regex::new(EMAIL_PATTERN).unwrap();
    let email_valid = re.is_match(email);
    if !email_valid { return SignUpStatus::EmailIsInvalid; }
    
    let email_exists_opt = is_recovery_user_info_already_exists_by_email(email, state).await;
    if email_exists_opt.is_none() { return SignUpStatus::DBConnectionLost; }
    if email_exists_opt.unwrap() { return SignUpStatus::EmailAlreadyInUse; }
    
    return SignUpStatus::OK;
}

async fn is_invite_code_valid(invite_code : &str, state : &AppState) -> SignUpStatus {
    if invite_code.is_empty() { return SignUpStatus::InviteCodeIsEmpty; }

    let db_service = SQLiteDbService::new(state);
    let invite_code_exists_opt = db_service.exists_by_prop("invites", "invite_code", invite_code).await;
    if invite_code_exists_opt.is_none() { return SignUpStatus::DBConnectionLost; }
    if !invite_code_exists_opt.unwrap() { return SignUpStatus::InviteCodeDoesNotExists; }
    
    return SignUpStatus::OK;
    
} 

async fn create_user(login : &str, password : &str, nickname : &str, email : &str, invite_code : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);

    // check all params
    let invite_opt = db_service.get_one_by_prop("invites", "invite_code", invite_code, row_to_invite).await;
    if invite_opt.is_none() { return; }

    let account_id_opt = db_service.new_id("accounts").await;
    if account_id_opt.is_none() { return; }
    let account_id = account_id_opt.unwrap();


    let public_user_info_opt = db_service.new_id("public_user_infos").await;
    if public_user_info_opt.is_none() { return; }
    let public_user_info_id = public_user_info_opt.unwrap();

    let recovery_user_info_opt = db_service.new_id("recovery_user_infos").await;
    if recovery_user_info_opt.is_none() { return; }
    let recovery_user_info_id = recovery_user_info_opt.unwrap();

    // find "user" role id

    let user_role_opt = db_service.get_one_by_prop("roles", "name", "user", row_to_role).await;
    if user_role_opt.is_none() { return; }
    let user_role = user_role_opt.unwrap();
    let user_role_id = user_role.id();

    let role_user_info_id_opt = db_service.new_id("roles_user_infos").await;
    if role_user_info_id_opt.is_none() { return; }
    let role_user_info_id = role_user_info_id_opt.unwrap();

    // create user
    create_account(account_id.as_str(), login, password, state).await;
    create_public_user_info(public_user_info_id.as_str(), account_id.as_str(), nickname, "", state).await;
    create_recovery_user_info(recovery_user_info_id.as_str(), account_id.as_str(), email, "", state).await;
    create_roles_user_info(role_user_info_id.as_str(), account_id.as_str(), user_role_id, "", state).await;


    // delete invite if it was one use code
    let invite = invite_opt.unwrap();
    let one_use = invite.one_use();

    if one_use {
        let invite_id = invite.id();
        let _ = db_service.delete_one_by_prop("invites", "id", invite_id).await;
    }

}

pub async fn user_sign_up(login : &str, password : &str, confirm_password : &str, nickname : &str, email : &str, invite_code : &str, state : &AppState) -> Vec<SignUpStatus> {
    let mut result : Vec<SignUpStatus> = Vec::new();

    if password != confirm_password {
        result.push(SignUpStatus::PasswordDoesNotMatch);
    }
    result.push(is_login_valid(login, state).await);
    result.push(is_password_valid(password, state).await);
    result.push(is_nickname_valid(nickname, state).await);
    result.push(is_email_valid(email, state).await);
    result.push(is_invite_code_valid(invite_code, state).await);

    let data_valid = result.clone().into_iter().all(|s : SignUpStatus| -> bool { s == SignUpStatus::OK });
    if data_valid {
        create_user(login, password, nickname, email, invite_code, state).await;
    }
    return result;
}

pub async fn user_create_invite_code(invite_code : &str, one_use : bool, state : &AppState) -> ApiResponse {
    let db_service = SQLiteDbService::new(state);

    let invite_code_is_empty = invite_code.is_empty();

    if !invite_code_is_empty {
        let invite_code_exists = db_service.exists_by_prop("invites", "invite_code", invite_code).await.unwrap();
        if invite_code_exists {
            let err_msg = format!("Invite code already exists");
            return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
        }
    }

    let one_use_str : &str;
    if one_use {
        one_use_str = "true";
    } else {
        one_use_str = "false";
    }

    let id = db_service.new_id("invites").await.unwrap();

    let mut code : String = String::from(invite_code);
    if invite_code_is_empty {
        loop {
            code = generate_random_token();
            let code_exists = db_service.exists_by_prop("invites", "invite_code", code.as_str()).await;
            if code_exists.is_some_and(|b| {!b}) {
                break;
            }
        }
    }

    let props = vec!["id", "invite_code", "one_use"];
    let values = vec![vec![id.as_str(), code.as_str(), one_use_str]];

    let _ = db_service.insert("invites", props, values).await;
    return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(id).unwrap());
}

pub async fn init_admin_if_not_exists(state : &AppState) -> ApiResponse {
    
    // read admin config

    let conf = state.config.lock().await;

    let login_string = &conf.admin.login;
    let login = login_string.as_str();
    let password_string = &conf.admin.password;
    let password = password_string.as_str();
    let nickname_string = &conf.admin.nickname;
    let nickname = nickname_string.as_str();
    let email_string = &conf.admin.email;
    let email = email_string.as_str();


    // try get admin from base
    let (admin_exists_opt, account_id_string_opt) = is_admin_already_exists(state).await;
    if admin_exists_opt.is_none() {
        let err_msg = "No access to database";
        return  ApiResponse::error_from_str(err_msg);
    }

    // update admin if exists
    if admin_exists_opt.is_some_and(|b| {b}) {
        if account_id_string_opt.is_some() {
            let account_id_string = account_id_string_opt.unwrap();
            let account_id = account_id_string.as_str();
            
            let account = get_account_by_id(account_id, state).await.unwrap();
            let public_user_info = get_public_user_info_by_account_id(account_id, state).await.unwrap();
            let recovery_user_info = get_recovery_user_info_by_account_id(account_id, state).await.unwrap();

            // check info before update
            let login_check = is_login_valid(login, state).await;
            let password_check = is_password_valid(password, state).await;
            let nickname_check = is_nickname_valid(nickname, state).await;
            let email_check = is_email_valid(email, state).await;
            
            let data_check = vec![login_check, nickname_check, password_check, email_check];
            
            // exclude exists errors if it is same with old data
            let is_data_valid = data_check.iter().all(|&s : &SignUpStatus| { 
                s == SignUpStatus::OK || 
                s == SignUpStatus::LoginExists && account.login() == login || 
                s == SignUpStatus::NicknameExists && public_user_info.nickname() == nickname || 
                s == SignUpStatus::EmailAlreadyInUse && recovery_user_info.email() == email});
            
            if is_data_valid {
                // update admin account data
                if account.login() != login { set_account_login(account_id, login, state).await; }
                let same_password = validate_hash(password, account.passwrod_salt(), account.password_hash());
                if !same_password { set_account_password(account_id, password, state).await; }
                if public_user_info.nickname() != nickname { set_nickname(account_id, nickname, state).await };
                if recovery_user_info.email() != email { set_email(account_id, email, state).await };

                let warn_msg = "Admin account info was updated.";
                return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(warn_msg).unwrap());
            } else {
                let warn_msg = "Admin account found, but info was not updated. Check admin config."; 
                return ApiResponse::new(ApiResponseStatus::WARNING, serde_json::to_value(warn_msg).unwrap());
            }
        }
        let err_msg = "Account found but id not!";
        return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap());
    }
    

    let one_use_code_string_opt = create_random_invite_code_safe(true, state).await;
    if one_use_code_string_opt.is_none() {
        let err_msg = "Cant create random invite code for admin sign up";
        return ApiResponse::error_from_str(err_msg);
    }
    let one_use_code_string = one_use_code_string_opt.unwrap();
    let one_use_code = one_use_code_string.as_str();

    let results = user_sign_up(login, password, password, nickname, email, one_use_code, state).await;

    let data_valid = results.clone().into_iter().all(|s : SignUpStatus| -> bool { s == SignUpStatus::OK });
    if data_valid {
        add_admin_role_to_admin(login, &state.clone()).await;
        let msg = "Admin created";
        return ApiResponse::new(ApiResponseStatus::OK, serde_json::to_value(msg).unwrap())
    }

    let error_msgs = sign_up_error_description_map();
    let err_msg : Vec<String> = results.iter().map(|&r| error_msgs.get(&r).unwrap().clone()).collect();
    return ApiResponse::new(ApiResponseStatus::ERROR, serde_json::to_value(err_msg).unwrap())

}

pub async fn is_admin_already_exists(state : &AppState) -> (Option<bool>, Option<String>) {
    let db_service = SQLiteDbService::new(state);
    let admin_role_opt = db_service.get_one_by_prop("roles", "name", "administrator", row_to_role).await;
    if admin_role_opt.is_none() { return (None, None); }
    let admin_role = admin_role_opt.unwrap();
    let admin_role_id = admin_role.id();
    let role_exists_opt = db_service.exists_by_prop("roles_user_infos", "role_id", admin_role_id).await;
    if role_exists_opt.is_none() { return (None, None); }

    let role_opt = db_service.get_one_by_prop("roles_user_infos", "role_id", admin_role_id, row_to_roles_user_info).await;
    if role_opt.is_none() { return (Some(false), None); }
    let role = role_opt.unwrap();
    let account_id = role.account_id();

    let account_exists_opt = db_service.exists_by_prop("accounts", "id", account_id).await;
    if account_exists_opt.is_none() { return (None, None); }
    if account_exists_opt.is_some_and(|b| {!b}) { return (Some(false), None);}

    for table_name in vec!["public_user_infos", "recovery_user_infos"] {
        let exists_opt = db_service.exists_by_prop(table_name, "account_id", account_id).await;
        if exists_opt.is_none() { return (None, None); }
        if exists_opt.is_some_and(| b | {!b}) { return (Some(false), None); }
    }
    return (Some(true), Some(String::from(account_id)));
}

pub async fn add_admin_role_to_admin(login : &str, state : &AppState) {
    let db_service = SQLiteDbService::new(state);
    let admin = db_service.get_one_by_prop("accounts", "login", login, row_to_account).await.unwrap();
    let admin_role = db_service.get_one_by_prop("roles", "name", "administrator", row_to_role).await.unwrap();
    let roles_user_info_id = db_service.new_id("roles_user_infos").await.unwrap();
    let _ = db_service.delete_one_by_prop("roles_user_infos", "account_id", admin.id()).await;
    create_roles_user_info(roles_user_info_id.as_str(), admin.id(), admin_role.id(), "", &state).await;
}