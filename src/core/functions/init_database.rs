use crate::{core::functions::execute_script_template_wo_return, AppState};

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

pub async fn init_database(state : &AppState) -> () {
    create_account_table(state).await;
    create_client_table(state).await;
    create_account_session_table(state).await;
    create_auth_code_table(state).await;
    create_public_user_info_table(state).await;
    create_recovery_user_info_table(state).await;
}