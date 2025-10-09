use std::{any::Any, future::Future, num::NonZeroU32, process::Output};

use async_fn_traits::{AsyncFn2, AsyncFnMut2};
use data_encoding::BASE64URL;
use ring::{digest, pbkdf2, rand::{self, SecureRandom}};
use sqlx::{sqlite::SqliteRow, Executor, Row};
use uuid::Uuid;

use crate::{core::data_model::traits::ILocalObject, santa::data_model::traits::IMember, AppState};

pub fn generate_random_token() -> String {
    const TOKEN_LEN : usize = 64;
    let rng = rand::SystemRandom::new();
    let mut token = [0u8; TOKEN_LEN];
    let _ = rng.fill(&mut token);
    return BASE64URL.encode(&token).replace("=", "");
}

pub async fn generate_id() -> String {
    return String::from(Uuid::new_v4());
}

pub fn hash_password_with_salt(password : &str, salt : &str) -> String {
    const CREDENTIAL_LEN : usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    
    let salt_bytes_vec = BASE64URL.decode(&salt.as_bytes()).unwrap();
    let salt_bytes = salt_bytes_vec.as_slice();
    let mut pwd_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512, n_iter, 
        &salt_bytes, password.as_bytes(),
        &mut pwd_hash
    );
    return BASE64URL.encode(&pwd_hash);
}

pub fn hash_password(password : &str) -> [String; 2] {
    const CREDENTIAL_LEN : usize = digest::SHA512_OUTPUT_LEN;
    let rng = rand::SystemRandom::new();
    let mut pwd_salt = [0u8; CREDENTIAL_LEN];
    let _ = rng.fill(&mut pwd_salt);
    let encoded_salt = BASE64URL.encode(&pwd_salt);
    return [hash_password_with_salt(&password, &encoded_salt), encoded_salt];
}

pub fn validate_hash(plain_password : &str, password_salt : &str, hashed_password : &str) -> bool {
    let hash_for_validate = hash_password_with_salt(plain_password, password_salt);
    return hash_for_validate == hashed_password;
}

pub async fn render_query_template(template_name : &str, context : &tera::Context, state : &AppState) -> String {
    // merge current and global contexts into one
    let mut extended_context = tera::Context::new();
    extended_context.extend(context.clone());
    extended_context.extend(state.context.lock().await.clone());

    // get sql query from template and extended context
    return state.tera.lock().await.render(template_name, &extended_context).unwrap();
}

pub async fn execute_command_wo_return(command : &str, state : &AppState) -> () {
    state.db.lock().await.execute(command).await.unwrap();
}

pub async fn execute_script_template_wo_return(template_name : &str, context : &tera::Context, state : &AppState)  -> () {
    let command = render_query_template(&template_name, &context, &state).await;
    execute_command_wo_return(command.as_str(), state).await;
}

pub async fn get_one_item_from_command<T>(command : &str, state : &AppState, transform_func : fn(&SqliteRow) -> T) -> Option<T> where T : ILocalObject {
    let conn = state.db.lock().await;
    let result = match conn.fetch_optional(command).await {
        Ok(o) => o,
        Err(_) => None
    };
    if result.is_some() {
        let obj = transform_func(&result.unwrap());
        return Some(obj);
    } else {
        return None;
    }
}

pub async fn get_many_items_from_command<T>(command : &str, state : &AppState, transform_func : fn(&SqliteRow) -> T) -> Option<Vec<T>> where T : ILocalObject {
    let conn = state.db.lock().await;
    let result = match conn.fetch_all(command).await {
        Ok(o) => Some(o),
        Err(_) => None,
    };

    if result.is_some() {
        let unwrap_result = result.unwrap();
        let v = unwrap_result.iter().map(transform_func).collect();
        return Some(v);
    } else {
        return None;
    }
}

pub async fn command_result_exists(command : &str, state : &AppState) -> bool {
    let conn = state.db.lock().await;
    let result = conn.fetch_one(command).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn new_id_safe<F>(check_func : F, state : &AppState) -> String where F : for<'a, 'b> AsyncFn2<&'a str, &'b AppState, Output = Option<bool>> {
    let mut new_id : String;
    loop {
        new_id = generate_id().await;
        let is_object_id_already_exists = check_func(new_id.as_str(), state).await;
        if is_object_id_already_exists.is_some_and(|b| {!b}) { break; }
    }
    return new_id;
}