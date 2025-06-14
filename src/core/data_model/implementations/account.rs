use serde::{Deserialize, Serialize};

use crate::core::{data_model::traits::{IAccount, ILocalObject}};

#[derive(Serialize, Deserialize)]
pub struct Account {
    id : String,
    login : String,
    password_hash : String,
    password_salt : String
}

impl ILocalObject for Account {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IAccount for Account {

    fn new(id : &str, login : &str, password_hash : &str, password_salt : &str) -> Self {
        return Account {
            id: String::from(id),
            login: String::from(login),
            password_hash: String::from(password_hash),
            password_salt: String::from(password_salt),
        };
    }

    fn login(&self) -> &str { self.login.as_str() }

    fn password_hash(&self) -> &str { self.password_hash.as_str() }

    fn passwrod_salt(&self) -> &str { self.password_salt.as_str() }

    fn set_login(&mut self, login : &str) -> () { self.login = String::from(login) }

    fn set_password_hash(&mut self, password_hash : &str) -> () { self.password_hash = String::from(password_hash) }
    
    fn set_password_salt(&mut self, password_salt : &str) -> () { self.password_salt = String::from(password_salt) }
}