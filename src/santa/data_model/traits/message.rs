use chrono::{DateTime, Utc};

use crate::{core::data_model::traits::IAccountRelated, santa::data_model::traits::IRoomRelated};

pub trait IMessage : IAccountRelated + IRoomRelated {

    fn new(id : &str, text_content : &str, account_id : &str, room_id : &str, creation_date : DateTime<Utc>) -> Self;

    fn text_content(&self) -> &str;
    fn creation_date(&self) -> DateTime<Utc>;

    fn set_text_content(&mut self, text_content : &str) -> ();
    fn set_creation_date(&mut self, creation_date : DateTime<Utc>) -> ();

}