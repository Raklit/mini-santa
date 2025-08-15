use std::cmp::min;

use sqlx::{Row, sqlite::SqliteRow, Executor};
use uuid::Uuid;

use crate::{core::{data_model::traits::ILocalObject}, AppState};

pub fn escape_string(string : &str) -> String {
    let mut result : String = String::from(string);
    result = result.replace("\\", "\\\\");
    result = result.replace("\"", "\\\"");
    result = result.replace("\'", "\\\'");
    result = result.replace(";", "\\;");
    return result;
}

pub trait IDbService {

    fn new(state : &AppState) -> Self;

    async fn insert_unsafe(&self, table_name : &str, props : Vec<&str>, values : Vec<Vec<String>>) -> Option<usize>;

    async fn insert(&self, table_name : &str, props : Vec<&str>, values : Vec<Vec<&str>>) -> Option<usize> {
        let esc_table_name = escape_string(table_name);
        let esc_props : Vec<String> = props.iter().map(|s| -> String {escape_string(s)}).collect();
        let esc_props_str = esc_props.iter().map(|s| -> &str {s.as_str()}).collect();
        let mut esc_values :Vec<Vec<String>> = Vec::new();
        for vec in values {
           let temp = vec.iter().map(|s| { escape_string(s) }).collect();
            esc_values.push(temp);
        }
        return self.insert_unsafe(esc_table_name.as_str(), esc_props_str, esc_values).await;
    }

    async fn update_unsafe(&self, table_name : &str, key_prop : &str, key_value : &str, props : Vec<&str>, values : Vec<&str>);

    async fn update(&self, table_name : &str, key_prop : &str, key_value : &str, props : Vec<&str>, values : Vec<&str>) {
        let esc_table_name = escape_string(table_name);
        let esc_key_prop = escape_string(key_prop);
        let esc_key_value = escape_string(key_value);
        let esc_props : Vec<String> = props.iter().map(|s| -> String {escape_string(s)}).collect();
        let esc_props_str = esc_props.iter().map(|s| {s.as_str()}).collect();
        let esc_values : Vec<String> = values.iter().map(|s| -> String {escape_string(s)}).collect();
        let esc_values_str = esc_values.iter().map(|s| {s.as_str()}).collect(); 
        self.update_unsafe(esc_table_name.as_str(), esc_key_prop.as_str(), esc_key_value.as_str(), esc_props_str, esc_values_str).await;
    }

    async fn get_all_unsafe<T>(&self, table_name : &str, transform_func : fn(&SqliteRow) -> T) -> Option<Vec<T>> where T : ILocalObject;

    async fn get_all<T>(&self, table_name : &str, transform_func : fn(&SqliteRow) -> T) -> Option<Vec<T>> where T : ILocalObject {
        let esc_table_name = escape_string(table_name);
        return self.get_all_unsafe(esc_table_name.as_str(), transform_func).await;
    }

    async fn get_many_by_prop_unsafe<T>(&self, table_name : &str, prop : &str, values : Vec<&str>, transform_func : fn(&SqliteRow) -> T) -> Option<Vec<T>> where T : ILocalObject;

    async fn get_many_by_prop<T>(&self, table_name : &str, prop : &str, values : Vec<&str>, transform_func : fn(&SqliteRow) -> T) -> Option<Vec<T>> where T : ILocalObject {
        let esc_table_name = escape_string(table_name);
        let esc_prop = escape_string(prop);
        let esc_values : Vec<String> = values.iter().map(|s| -> String {escape_string(s)}).collect();
        let esc_values_str = esc_values.iter().map(|s| -> &str {s.as_str()}).collect();
        return self.get_many_by_prop_unsafe(esc_table_name.as_str(), esc_prop.as_str(), esc_values_str, transform_func).await;
    }

    async fn get_one_by_prop<T>(&self, table_name : &str, prop : &str, value : &str, transform_func : fn(&SqliteRow) -> T) -> Option<T> where T : ILocalObject + Clone {
        let values = vec![value];
        let v : Option<Vec<T>> = self.get_many_by_prop(table_name, prop, values, transform_func).await;
        if v.is_none() { return None; }
        let unwrap_v = v.unwrap();
        if unwrap_v.is_empty() { return None; }
        return Some(unwrap_v[0].clone());
    }

    async fn delete_many_by_prop_unsafe(&self, table_name : &str, prop : &str, values : Vec<&str>) -> Option<usize>;

    async fn delete_many_by_prop(&self, table_name : &str, prop : &str, values : Vec<&str>) -> Option<usize> {
        let esc_table_name = escape_string(table_name);
        let esc_prop = escape_string(prop);
        let esc_values : Vec<String> = values.iter().map(|s| -> String {escape_string(s)}).collect();
        let esc_values_str = esc_values.iter().map(|s| -> &str {s.as_str()}).collect();
        return self.delete_many_by_prop_unsafe(esc_table_name.as_str(), esc_prop.as_str(), esc_values_str).await;
    }

    async fn delete_one_by_prop(&self, table_name : &str, prop : &str, value : &str) {
        let values = vec![value];
        self.delete_many_by_prop(table_name, prop, values).await;
    }

    async fn exists_by_prop_unsafe(&self, table_name : &str, prop : &str, value : &str) -> Option<bool>;

    async fn exists_by_prop(&self, table_name : &str, prop : &str, value : &str) -> Option<bool> {
        let esc_table_name = escape_string(table_name);
        let esc_prop = escape_string(prop);
        let esc_value = escape_string(value);
        return self.exists_by_prop_unsafe(esc_table_name.as_str(), esc_prop.as_str(), esc_value.as_str()).await;
    }

    async fn generate_id_unsafe(&self) -> String {
        return String::from(Uuid::new_v4());
    }

    async fn new_id(&self, table_name : &str) -> Option<String> {
        let esc_table_name = escape_string(table_name);
        let esc_table_name_str = esc_table_name.as_str();
        let mut new_id_string : String;
        loop {
            new_id_string = self.generate_id_unsafe().await;
            let exists = self.exists_by_prop_unsafe(esc_table_name_str, "id", new_id_string.as_str()).await;
            if exists.is_none() { return None; }
            if !exists.unwrap() { break; }
        }
        return Some(new_id_string);
    }
}

pub struct SQLiteDbService {
    state : AppState
}

impl IDbService for SQLiteDbService {

    fn new(state : &AppState) -> Self {
        return SQLiteDbService {
            state : state.clone()
        };
     }

    async fn insert_unsafe(&self, table_name : &str, props : Vec<&str>, values : Vec<Vec<String>>) -> Option<usize> {
        let props_quoted : Vec<String> = props.iter().map(| s | -> String {
            let result = format!("\"{s}\"");
            return result;
        }).collect();
        let props_str = props_quoted.join(", ");

        let mut temp_vec = Vec::<String>::new();
        for value_vec in values {
            let temp_quoted : Vec<String> = value_vec.iter().map(| s| -> String {
                let result = format!("\'{s}\'");
                return result; 
            }).collect();
            let temp_commas = temp_quoted.join(", ");
            let temp = format!("({temp_commas})");
            temp_vec.push(temp);
        }
        let values_str = temp_vec.join(", ");

        let query = format!("INSERT INTO \"{table_name}\" ({props_str}) VALUES {values_str}");
        let conn = self.state.db.lock().await;
        let result : Option<usize> = match conn.execute(query.as_str()).await {
            Ok(o) => Some(o.rows_affected().try_into().unwrap()),
            Err(_) => None,
        };
        return result;
    }

    async fn update_unsafe(&self, table_name : &str, key_prop : &str, key_value : &str, props : Vec<&str>, values : Vec<&str>) {
        let n = min(props.len(), values.len());
        if n == 0 { return; }

        let mut set_lines = Vec::<String>::new();
        for i in 0..n {
            let prop = props.get(i).unwrap();
            let val =  values.get(i).unwrap();
            let temp = format!("\"{prop}\" = \'{val}\'");
            set_lines.push(temp);
        }
        let set_lines_str = set_lines.join(", ");
        let query = format!("UPDATE \"{table_name}\" SET {set_lines_str} WHERE \"{key_prop}\" = \'{key_value}\'");
        let conn = self.state.db.lock().await;
        let _ = conn.execute(query.as_str()).await;

    }

    async fn get_all_unsafe<T>(&self, table_name : &str, transform_func : fn(&SqliteRow) -> T) -> Option<Vec<T>> where T : ILocalObject {
        let query = format!("SELECT * FROM \"{table_name}\"");
        let conn = self.state.db.lock().await;
        let query_result = match conn.fetch_all(query.as_str()).await {
            Ok(o) => Some(o),
            Err(_) => None
        };
        if query_result.is_none() { return None; }
        let rows = query_result.unwrap();
        let objs = rows.iter().map(transform_func).collect();
        return Some(objs);
    }

    async fn get_many_by_prop_unsafe<T>(&self, table_name : &str, prop : &str, values : Vec<&str>, transform_func : fn(&SqliteRow) -> T) -> Option<Vec<T>> where T : ILocalObject {
        let values_quoted : Vec<String> = values.iter().map(| s | -> String {
            let result = format!("\'{s}\'");
            return result;
        }).collect();
        let values_str = values_quoted.join(", ");
        let query = format!("SELECT * FROM \"{table_name}\" WHERE \"{prop}\" IN ({values_str})");

        let conn = self.state.db.lock().await;
        let query_result = match conn.fetch_all(query.as_str()).await {
            Ok(o) => Some(o),
            Err(_) => None
        };
        if query_result.is_none() { return None; }
        let rows = query_result.unwrap();
        let objs = rows.iter().map(transform_func).collect();
        return Some(objs);

    }

    async fn delete_many_by_prop_unsafe(&self, table_name : &str, prop : &str, values : Vec<&str>) -> Option<usize> {
        let values_quoted : Vec<String> = values.iter().map(| s | -> String {
            let result = format!("\'{s}\'");
            return result;
        }).collect();
        let values_str = values_quoted.join(", ");
        let query = format!("DELETE FROM \"{table_name}\" WHERE \"{prop}\" IN ({values_str})");
        let conn = self.state.db.lock().await;
        let result : Option<usize> = match conn.execute(query.as_str()).await {
            Ok(o) => Some(o.rows_affected().try_into().unwrap()),
            Err(_) => None,
        };
        return result;
    }
    
    async fn exists_by_prop_unsafe(&self, table_name : &str, prop : &str, value : &str) -> Option<bool> {
        let query = format!("SELECT EXISTS(SELECT 1 FROM \"{table_name}\" WHERE \"{prop}\" = \'{value}\') AS row_exists;");
        let conn = self.state.db.lock().await;
        let result = match conn.fetch_one(query.as_str()).await {
            Ok(o) => Some(o),
            Err(_) => None,
        };
        if result.is_none() { return None; }
        let row = result.unwrap();
        let val : u8 = row.get("row_exists");
        return Some(val == 1);
    }
}