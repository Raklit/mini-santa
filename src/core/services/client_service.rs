use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Executor};

use crate::core::data_model::implementations::Client;
use crate::core::data_model::traits::IClient;
use crate::core::functions::{execute_script_template_wo_return, hash_password, render_query_template};
use crate::AppState;

fn row_to_client(row : &SqliteRow) -> Client {
    let id : &str = row.get("id");
    let client_name : &str = row.get("client_name");
    let password_hash : &str = row.get("password_hash");
    let password_salt : &str = row.get("password_salt");
    return Client {
        id : String::from(id),
        client_name : String::from(client_name),
        password_hash : String::from(password_hash),
        password_salt : String::from(password_salt)
    }
}

pub async fn is_client_already_exists_by_id_or_client_name(id : &str, client_name : &str, state : &AppState) -> bool {
    const EXISTS_CLIENT_BY_ID_OR_CLIENT_NAME_TEMPLATE : &str = "database_scripts/client/exists_client_by_id_or_client_name.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("cliennt_name", &client_name);
    let command = render_query_template(EXISTS_CLIENT_BY_ID_OR_CLIENT_NAME_TEMPLATE, &context, &state);
    let result = state.db.fetch_one(command.as_str()).await.unwrap();
    let val : u8 = result.get(0);
    return val == 1;
}

pub async fn create_client(id : &str, client_name : &str, password : &str, state : &AppState) -> () {
    let hashed_password = hash_password(&password);
    let client = Client {
         id : String::from(id),
         client_name : String::from(client_name),
         password_hash : hashed_password[0].clone(),
         password_salt : hashed_password[1].clone()
     };
 
     let mut context = tera::Context::new();
     context.insert("id", &client.id.as_str());
     context.insert("client_name", &client.client_name.as_str());
     context.insert("password_hash", &client.password_hash.as_str());
     context.insert("password_salt", &client.password_salt.as_str());
 
     let client_exists = is_client_already_exists_by_id_or_client_name(&client.id.as_str(), &client.client_name.as_str(), &state).await;
     if !client_exists {
         const CREATE_CLIENT_TEMPLATE : &str = "database_scripts/client/create_client.sql";
         execute_script_template_wo_return(CREATE_CLIENT_TEMPLATE, &context, &state).await;
     }
 }

pub async fn get_client_by_id(id : &str, state : &AppState) -> impl IClient {
    const GET_CLIENT_BY_ID_TEMPLATE : &str = "database_scripts/client/get_client_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    
    let command = render_query_template(GET_CLIENT_BY_ID_TEMPLATE, &context, &state);
    let result = state.db.fetch_one(command.as_str()).await.unwrap();
    
    return row_to_client(&result);
}

pub async fn get_client_by_client_name(client_name : &str, state : &AppState) -> impl IClient {
    const GET_CLIENT_BY_CLIENT_NAME_TEMPLATE : &str = "database_scripts/client/get_client_by_client_name.sql";
    let mut context = tera::Context::new();
    context.insert("client_name", &client_name);
    
    let command = render_query_template(GET_CLIENT_BY_CLIENT_NAME_TEMPLATE, &context, &state);
    let result = state.db.fetch_one(command.as_str()).await.unwrap();
    
    return row_to_client(&result);
}

pub async fn set_client_name(id : &str, client_name : &str, state : &AppState) -> () {
    const SET_CLIENT_NAME_TEMPLATE : &str = "database_scripts/client/set_client_name.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("client_name", &client_name);
    execute_script_template_wo_return(SET_CLIENT_NAME_TEMPLATE, &context, &state).await;
}

pub async fn set_client_password(id : &str, password : &str, state : &AppState) -> () {
    let hashed_password = hash_password(&password);

    const SET_CLIENT_PASSWORD_TEMPLATE : &str = "database_scripts/client/set_client_password.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    context.insert("password_hash", hashed_password[0].as_str());
    context.insert("password_salt", hashed_password[1].as_str());
    execute_script_template_wo_return(SET_CLIENT_PASSWORD_TEMPLATE, &context, &state).await;
}

pub async fn delete_client_by_id(id : &str, state : &AppState) -> () {
    const DELETE_CLIENT_BY_ID_TEMPLATE : &str = "database_scripts/client/delete_client_by_id.sql";
    let mut context = tera::Context::new();
    context.insert("id", &id);
    execute_script_template_wo_return(DELETE_CLIENT_BY_ID_TEMPLATE, &context, &state).await;
}

pub async fn delete_client_by_client_name(client_name : &str, state : &AppState) -> () {
    const DELETE_CLIENT_BY_CLIENT_NAME_TEMPLATE : &str = "database_scripts/client/delete_client_by_client_name.sql";
    let mut context = tera::Context::new();
    context.insert("client_name", &client_name);
    execute_script_template_wo_return(DELETE_CLIENT_BY_CLIENT_NAME_TEMPLATE, &context, &state).await;
}