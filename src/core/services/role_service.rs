use sqlx::{Row, sqlite::SqliteRow};

use crate::{core::{data_model::{implementations::Role, traits::IRole}, services::{IDbService, SQLiteDbService}}, AppState};

pub fn row_to_role(row : &SqliteRow) -> Role {
    let id : &str = row.get("id");
    let name : &str = row.get("name");
    let tags : &str = row.get("tags");

    return Role::new(id, name, tags);
}

pub async fn is_role_already_exists_by_id(id : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("roles", "id", id).await;
}

pub async fn is_role_already_exists_by_name(name : &str, state : &AppState) -> Option<bool> {
    let db_service = SQLiteDbService::new(state);
    return db_service.exists_by_prop("roles", "name", name).await;
}

pub async fn create_role(id : &str, name : &str, tags : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["id", "name", "tags"];
    let values = vec![vec![id, name, tags]];
    let _ = db_service.insert("roles", props, values).await;
 }

pub async fn get_role_by_id(id : &str, state : &AppState) -> Option<impl IRole> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("roles", "id", id, row_to_role).await;
}

pub async fn get_role_by_name(name : &str, state : &AppState) -> Option<impl IRole> {
    let db_service = SQLiteDbService::new(state);
    return db_service.get_one_by_prop("roles", "name", name, row_to_role).await;
}

pub async fn set_name(id : &str, name : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["name"];
    let values = vec![name];
    let _ = db_service.update("roles", "id", id, props, values).await;
}

pub async fn set_tags(id : &str, tags : &str, state : &AppState) -> () {
    let db_service = SQLiteDbService::new(state);
    let props = vec!["tags"];
    let values = vec![tags];
    let _ = db_service.update("roles", "id", id, props, values).await;
}