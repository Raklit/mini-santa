use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::{IClient, ILocalObject};

#[derive(Serialize, Deserialize)]
pub struct Client {
    id : String,
    client_name : String,
    password_hash : String,
    password_salt : String
}

impl ILocalObject for Client {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IClient for Client {

    fn new(id : &str, client_name : &str, password_hash : &str, password_salt : &str) -> Client {
        return Client {
            id: String::from(id),
            client_name: String::from(client_name),
            password_hash: String::from(password_hash),
            password_salt: String::from(password_salt)
        };
    }

    fn client_name(&self) -> &str { self.client_name.as_str() }

    fn password_hash(&self) -> &str { self.password_hash.as_str() }

    fn password_salt(&self) -> &str { self.password_salt.as_str() }

    fn set_client_name(&mut self, client_name : &str) -> () { self.client_name = String::from(client_name) }

    fn set_password_hash(&mut self, password_hash : &str) -> () { self.password_hash = String::from(password_hash) }
    
    fn set_password_salt(&mut self, password_salt : &str) -> () { self.password_salt = String::from(password_salt) }
}