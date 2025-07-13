use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::{IAccountRelated, ILocalObject};
use crate::santa::data_model::traits::{IMessage, IRoomRelated};

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    id : String,
    text_content : String,
    account_id : String,
    room_id : String,
    creation_date : DateTime<chrono::Utc>

}

impl ILocalObject for Message {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IAccountRelated for Message {
    fn account_id(&self) -> &str { self.account_id.as_str() }

    fn set_account_id(&mut self, account_id : &str) -> () { self.account_id = String::from(account_id); }
}

impl IRoomRelated for Message {
    fn room_id(&self) -> &str { self.room_id.as_str() }

    fn set_room_id(&mut self, room_id : &str) -> () { self.room_id = String::from(room_id); }
}

impl IMessage for Message {

    fn new(id : &str, text_content : &str, account_id : &str, room_id : &str, creation_date : DateTime<Utc>) -> Self {
        return Message {
            id : String::from(id),
            text_content : String::from(text_content),
            account_id : String::from(account_id),
            room_id: String::from(room_id),
            creation_date : creation_date,
        };
    }
    
    fn text_content(&self) -> &str { self.text_content.as_str() }
    
    fn creation_date(&self) -> DateTime<Utc> { self.creation_date }
    
    fn set_text_content(&mut self, text_content : &str) -> () { self.text_content = String::from(text_content); }
    
    fn set_creation_date(&mut self, creation_date : DateTime<Utc>) -> () { self.creation_date = creation_date; }
}