use crate::{core::functions::execute_script_template_wo_return, AppState};

async fn create_pool_table(state : &AppState) -> () {
    const CREATE_POOL_TABLE_TEMPLATE: &str = "database_scripts/tables/create_pool_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_POOL_TABLE_TEMPLATE, &context, &state).await;
}

async fn create_room_table(state : &AppState) -> () {
    const CREATE_ROOM_TABLE_TEMPLATE: &str = "database_scripts/tables/create_room_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_ROOM_TABLE_TEMPLATE, &context, &state).await;
}

async fn create_member_table(state : &AppState) -> () {
    const CREATE_MEMBER_TABLE_TEMPLATE: &str = "database_scripts/tables/create_member_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_MEMBER_TABLE_TEMPLATE, &context, &state).await;
}

async fn create_message_table(state : &AppState) -> () {
    const CREATE_MESSAGE_TABLE_TEMPLATE: &str = "database_scripts/tables/create_message_table.sql";
    let context = tera::Context::new();
    execute_script_template_wo_return(CREATE_MESSAGE_TABLE_TEMPLATE, &context, &state).await;
}

pub async fn santa_init_database(state : &AppState) -> () {
    create_pool_table(state).await;
    create_room_table(state).await;
    create_member_table(state).await;
    create_message_table(state).await;
}