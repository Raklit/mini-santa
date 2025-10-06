use sqlx::{sqlite::SqliteRow, Row};

use crate::{core::{data_model::{implementations::Invite, traits::IInvite}, functions::generate_random_token, services::{db_service, escape_string, IDbService, SQLiteDbService}}, AppState};

pub fn row_to_invite(row : &SqliteRow) -> Invite {
    let id : &str = row.get("id");
    let invite_code : &str = row.get("invite_code");
    let one_use_str : &str = row.get("one_use");
    let one_use = one_use_str.to_lowercase() == "true";
    return Invite::new(id, invite_code, one_use);
}

pub async fn is_invite_code_already_exists(invite_code : &str ,state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("invites", "code", invite_code).await;
}

pub async fn create_code(id : &str, invite_code : &str, is_one_use : bool, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    
    let is_one_use_str;
    if is_one_use {
        is_one_use_str = "true"
    } else {
        is_one_use_str = "false";
    }

    let result_opt = db_service.insert("invites", vec!["id", "code", "one_use"], vec![vec![id, invite_code, is_one_use_str]]).await;
    if result_opt.is_none() { return None; }
    let result = result_opt.unwrap();
    return Some(result != 0);
}


pub async fn create_random_invite_code_safe(is_one_use : bool, state : &AppState) -> Option<String> {
    let code_string_opt = generate_random_code_safe(state).await;
    if code_string_opt.is_none() { return None; }
    let code_string = code_string_opt.unwrap();
    let code = code_string.as_str();

    let db_service = SQLiteDbService::new(state);
    
    let id_string_opt = db_service.new_id("invites").await;
    if id_string_opt.is_none() { return None; }
    let id_string = id_string_opt.unwrap();
    let id = id_string.as_str();
    
    let result_opt = create_code(id, code, is_one_use, state).await;
    if result_opt.is_none_or(|b| {!b}) {
        return None;
    }
    return Some(String::from(code));
}

async fn generate_random_code_safe(state : &AppState) -> Option<String> {
    let mut temp_code_string = generate_random_token();
    loop {
        temp_code_string = generate_random_token();
        let temp_code = temp_code_string.as_str();
        let code_exists_opt = is_invite_code_already_exists(temp_code, state).await;
        if code_exists_opt.is_none() { return None; }
        if code_exists_opt.is_some_and(|b| {b}) { break; }
    };
    return Some(temp_code_string);
}