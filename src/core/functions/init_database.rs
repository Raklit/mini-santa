use crate::{core::{functions::execute_script_template_wo_return, services::{create_role, IDbService, SQLiteDbService}}, AppState};

async fn create_account_table(state : &AppState) -> () {
    const CREATE_ACCOUNT_TABLE_TEMPLATE: &str = "database_scripts/tables/create_account_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_ACCOUNT_TABLE_TEMPLATE, &context, &state).await;
}

async fn create_client_table(state : &AppState) -> () {
    const CREATE_CLIENT_TABLE_TEMPLATE: &str = "database_scripts/tables/create_client_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_CLIENT_TABLE_TEMPLATE, &context, &state).await;
}

async fn create_account_session_table(state : &AppState) -> () {
    const CREATE_ACCOUNT_SESSION_TABLE_TEMPLATE: &str = "database_scripts/tables/create_account_session_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_ACCOUNT_SESSION_TABLE_TEMPLATE, &context, &state).await;
}

async fn create_auth_code_table(state : &AppState) -> () {
    const CREATE_AUTH_CODE_TABLE_TEMPLATE: &str = "database_scripts/tables/create_auth_code_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_AUTH_CODE_TABLE_TEMPLATE, &context, &state).await;
}

async fn create_roles_user_info_table(state : &AppState) -> () {
    const CREATE_ROLES_USER_INFO_TABLE_TEMPLATE : &str = "database_scripts/tables/create_roles_user_info_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_ROLES_USER_INFO_TABLE_TEMPLATE, &context, &state).await;

}

async fn create_role_table(state : &AppState) -> () {
    const CREATE_ROLE_TABLE_TEMPLATE : &str = "database_scripts/tables/create_role_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_ROLE_TABLE_TEMPLATE, &context, &state).await;
}

async fn create_public_user_info_table(state : &AppState) -> () {
    const CREATE_PUBLIC_USER_INFO_TABLE_TEMPLATE: &str = "database_scripts/tables/create_public_user_info_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_PUBLIC_USER_INFO_TABLE_TEMPLATE, &context, &state).await;
}

async fn create_recovery_user_info_table(state : &AppState) -> () {
    const CREATE_RECOVERY_USER_INFO_TABLE_TEMPLATE: &str = "database_scripts/tables/create_recovery_user_info_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_RECOVERY_USER_INFO_TABLE_TEMPLATE, &context, &state).await;
}

async fn create_invite_table(state : &AppState) -> () {
    const CREATE_INVITE_TABLE_TEMPLATE: &str = "database_scripts/tables/create_invite_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_INVITE_TABLE_TEMPLATE, &context, &state).await;
}

pub async fn core_init_database(state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);

    create_account_table(state).await;
    create_client_table(state).await;
    create_account_session_table(state).await;
    create_auth_code_table(state).await;
    create_roles_user_info_table(state).await;
    create_public_user_info_table(state).await;
    create_recovery_user_info_table(state).await;
    create_role_table(state).await;
    create_invite_table(state).await;

    let mut role_id : String;
    if db_service.exists_by_prop("roles", "name", "administrator").await.is_some_and(|b| {!b}) {
        role_id = db_service.new_id("roles").await.unwrap();
        create_role(role_id.as_str(), "administrator", "god", state).await;
    }
    if db_service.exists_by_prop("roles", "name", "moderator").await.is_some_and(|b| {!b}) {
        role_id = db_service.new_id("roles").await.unwrap();
        create_role(role_id.as_str(), "moderator", "moderator", state).await;
    }
    if db_service.exists_by_prop("roles", "name", "user").await.is_some_and(|b| {!b}) {
        role_id = db_service.new_id("roles").await.unwrap();
        create_role(role_id.as_str(), "user", "user", state).await;
    }
}