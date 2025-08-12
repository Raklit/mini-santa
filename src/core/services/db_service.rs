use std::cmp::min;

use sqlx::{any::AnyRow, Executor};

use crate::AppState;

pub fn escape_string(string : &str) -> String {
    let mut result : String = String::from(string);
    result = result.replace("\\", "\\\\");
    result = result.replace("\"", "\\\"");
    result = result.replace("\'", "\\\'");
    result = result.replace(";", "\\;");
    return result;
}

pub trait IDbService {

    async fn new(state : &AppState) -> Self;

    async fn insert_unsafe(&self, table_name : String, props : Vec<String>, values : Vec<Vec<String>>) -> Option<usize>;

    async fn insert(&self, table_name : String, props : Vec<String>, values : Vec<Vec<String>>) -> Option<usize> {
        let esc_table_name = escape_string(table_name.as_str());
        let esc_props : Vec<String> = props.iter().map(|s| -> String {escape_string(s)}).collect();
        let mut esc_values : Vec<Vec<String>> = Vec::new();
        for vec in values {
            esc_values.push(vec.iter().map(|s| -> String { escape_string(s) }).collect());
        }
        return self.insert_unsafe(esc_table_name, esc_props, esc_values).await;
    }

    async fn update_unsafe(&self, table_name : String, primary_key_prop : String, primary_key_value : String, props : Vec<String>, values : Vec<String>);

    async fn update(&self, table_name : String, primary_key_prop : String, primary_key_value : String, props : Vec<String>, values : Vec<String>) {
        let esc_table_name = escape_string(table_name.as_str());
        let esc_primary_key_prop = escape_string(primary_key_prop.as_str());
        let esc_primary_key_value = escape_string(primary_key_value.as_str());
        let esc_props : Vec<String> = props.iter().map(|s| -> String {escape_string(s)}).collect();
        let esc_values : Vec<String> = values.iter().map(|s| -> String {escape_string(s)}).collect();
        self.update_unsafe(esc_table_name, esc_primary_key_prop, esc_primary_key_value, esc_props, esc_values).await;
    }

    async fn get_many_by_prop_unsafe(&self, table_name : String, prop : String, values : Vec<String>) -> Option<Vec<AnyRow>>;

    async fn get_many_by_prop(&self, table_name : String, prop : String, values : Vec<String>) -> Option<Vec<AnyRow>> {
        let esc_table_name = escape_string(table_name.as_str());
        let esc_prop = escape_string(prop.as_str());
        let esc_values : Vec<String> = values.iter().map(|s| -> String {escape_string(s)}).collect();
        return self.get_many_by_prop_unsafe(esc_table_name, esc_prop, esc_values).await;
    }

    async fn get_one_by_prop(&self, table_name : String, prop : String, value : String) -> Option<AnyRow> {
        let values = vec![value];
        let v = self.get_many_by_prop(table_name, prop, values).await;
        if v.is_none() { return None; }
        let unwrap_v = v.unwrap();
        if unwrap_v.is_empty() { return None; }
        return Some(unwrap_v[0].clone());
    }

    async fn delete_many_by_prop_unsafe(&self, table_name : String, prop : String, values : Vec<String>) -> Option<usize>;

    async fn delete_many_by_prop(&self, table_name : String, prop : String, values : Vec<String>) -> Option<usize> {
        let esc_table_name = escape_string(table_name.as_str());
        let esc_prop = escape_string(prop.as_str());
        let esc_values : Vec<String> = values.iter().map(|s| -> String {escape_string(s)}).collect();
        return self.delete_many_by_prop_unsafe(esc_table_name, esc_prop, esc_values).await;
    }

    async fn delete_one_by_prop(&self, table_name : String, prop : String, value : String) {
        let values = vec![value];
        self.delete_many_by_prop(table_name, prop, values).await;
    }

    async fn exists_by_prop(&self, table_name : String, prop : String, value : String) -> bool {
        return self.get_one_by_prop(table_name, prop, value).await.is_some();
    }
}

pub struct SQLiteDbService {
    state : AppState
}

impl IDbService for SQLiteDbService {
    async fn new(state : &AppState) -> Self {
        return SQLiteDbService {
            state : state.clone()
        };
    }

    async fn insert_unsafe(&self, table_name : String, props : Vec<String>, values : Vec<Vec<String>>) -> Option<usize> {
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

    async fn update_unsafe(&self, table_name : String, primary_key_prop : String, primary_key_value : String, props : Vec<String>, values : Vec<String>) {
        
        let n = min(props.len(), values.len());
        if n == 0 { return; }

        let mut set_lines = Vec::<String>::new();
        for i in 0..n {
            let prop = props.get(i).unwrap().as_str();
            let val =  values.get(i).unwrap().as_str();
            let temp = format!("\"{prop}\" = \'{val}\'");
            set_lines.push(temp);
        }
        let set_lines_str = set_lines.join(", ");
        let query = format!("UPDATE \"{table_name}\" SET {set_lines_str} WHERE \"{primary_key_prop}\" = \'{primary_key_value}\'");
        let conn = self.state.db.lock().await;
        let _ = conn.execute(query.as_str()).await;

    }

    async fn get_many_by_prop_unsafe(&self, table_name : String, prop : String, values : Vec<String>) -> Option<Vec<AnyRow>> {
        let values_quoted : Vec<String> = values.iter().map(| s | -> String {
            let result = format!("\'{s}\'");
            return result;
        }).collect();
        let values_str = values_quoted.join(", ");
        let query = format!("SELECT * FROM \"{table_name}\" WHERE \"{prop}\" IN {values_str}");

        let conn = self.state.db.lock().await;
        let query_result = match conn.fetch_all(query.as_str()).await {
            Ok(o) => Some(o),
            Err(_) => None
        };
        if query_result.is_none() { return None; }
        let rows = query_result.unwrap();
        let mut result : Vec<AnyRow> = Vec::new();
        for row in rows {
            let temp = AnyRow::try_from(&row).unwrap();
            result.push(temp)
        }
        return Some(result);

    }

    async fn delete_many_by_prop_unsafe(&self, table_name : String, prop : String, values : Vec<String>) -> Option<usize> {
        let values_quoted : Vec<String> = values.iter().map(| s | -> String {
            let result = format!("\'{s}\'");
            return result;
        }).collect();
        let values_str = values_quoted.join(", ");
        let query = format!("DELETE FROM \"{table_name}\" WHERE \"{prop}\" IN {values_str}");
        let conn = self.state.db.lock().await;
        let result : Option<usize> = match conn.execute(query.as_str()).await {
            Ok(o) => Some(o.rows_affected().try_into().unwrap()),
            Err(_) => None,
        };
        return result;
    }
}