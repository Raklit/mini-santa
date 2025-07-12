use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::ILocalObject;
use crate::santa::data_model::traits::{IMessage, IRoomRelated};

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    id : String,
    text : String,
    room_id : String,
    is_send_by_mailer : bool,
    date : chrono::DateTime<chrono::Utc>

}

impl ILocalObject for Message {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IRoomRelated for Message {
    fn room_id(&self) -> &str { self.room_id.as_str() }

    fn set_room_id(&mut self, room_id : &str) -> () { self.room_id = String::from(room_id); }
}

impl IMessage for Message {

    fn new(id : &str, text : &str, room_id : &str, is_send_by_mailer : bool, date : DateTime<Utc>) -> Self {
        return Message {
            id : String::from(id),
            text : String::from(text),
            room_id: String::from(room_id),
            is_send_by_mailer : is_send_by_mailer,
            date : date,
        };
    }

    fn text(&self) -> &str { self.text.as_str() }

    fn is_send_by_mailer(&self) -> bool { self.is_send_by_mailer }

    fn date(&self) -> DateTime<Utc> { self.date }

    fn set_text(&mut self, text : &str) -> () { self.text = String::from(text) }

    fn set_is_send_by_mailer(&mut self, value : bool) -> () { self.is_send_by_mailer = value }

    fn set_date(&mut self, date : DateTime<Utc>) -> () { self.date = date }
    
    fn is_send_by_recipient(&self) -> bool { !self.is_send_by_mailer() }
}