use crate::{core::functions::execute_script_template_wo_return, AppState};

async fn create_account_table(state : &AppState) -> () {
    const CREATE_ACCOUNT_TABLE_TEMPLATE: &str = "database_scripts/tables/create_account_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_ACCOUNT_TABLE_TEMPLATE, &context, &state).await;
}

pub async fn init_database(state : &AppState) -> () {
    create_account_table(&state).await;
}