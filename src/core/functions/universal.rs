use std::num::NonZeroU32;

use data_encoding::BASE64URL;
use ring::{digest, pbkdf2, rand::{self, SecureRandom}};
use sqlx::Executor;

use crate::AppState;

pub fn generate_random_token() -> String {
    const TOKEN_LEN : usize = 64;
    let rng = rand::SystemRandom::new();
    let mut token = [0u8; TOKEN_LEN];
    let _ = rng.fill(&mut token);
    let result = BASE64URL.encode(&token);
    return result;
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
    let result = hash_for_validate == hashed_password;
    return hash_for_validate == hashed_password;
}

pub fn render_query_template(template_name : &str, context : &tera::Context, state : &AppState) -> String {
    // merge current and global contexts into one
    let mut extended_context = tera::Context::new();
    extended_context.extend(context.clone());
    extended_context.extend((&state.context).clone());

    // get sql query from temaplate and extended context
    return state.tera.render(template_name, &extended_context).unwrap();
}

pub async fn execute_script_template_wo_return(template_name : &str, context : &tera::Context, state : &AppState)  -> () {
    let create_account_table_command = render_query_template(&template_name, &context, &state);
    state.db.execute(create_account_table_command.as_str()).await.unwrap();
}