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

pub fn hash_password(password : &str) -> [String; 2] {
    const CREDENTIAL_LEN : usize = digest::SHA512_OUTPUT_LEN;
    let rng = rand::SystemRandom::new();
    let mut pwd_salt = [0u8; CREDENTIAL_LEN];
    let _ = rng.fill(&mut pwd_salt);

    let n_iter = NonZeroU32::new(100_000).unwrap();
    let mut pwd_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512, n_iter, 
        &pwd_salt, password.as_bytes(),
         &mut pwd_hash
    );
    let result = [BASE64URL.encode(&pwd_hash), BASE64URL.encode(&pwd_salt)];
    return result;
}

pub fn validate_hash(plain_password : &str, password_salt : &str, hashed_password : &str) -> bool {
    const CREDENTIAL_LEN : usize = digest::SHA512_OUTPUT_LEN;
    
    let pwd_salt_vec  = BASE64URL.decode(&password_salt.as_bytes()).unwrap();
    let pwd_salt = pwd_salt_vec.as_slice();
    let n_iter = NonZeroU32::new(100_000).unwrap();
    
    let mut hash_for_validate = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512, n_iter, 
        pwd_salt, &plain_password.as_bytes(), 
        &mut hash_for_validate
    );
    let str_for_validate = BASE64URL.encode(&hash_for_validate);

    return hashed_password == str_for_validate;
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