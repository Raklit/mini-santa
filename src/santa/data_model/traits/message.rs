use chrono::{DateTime, Utc};

use crate::santa::data_model::traits::IRoomRelated;

pub trait IMessage : IRoomRelated {

    fn new(id : &str, text : &str, room_id : &str, is_send_by_mailer : bool, date : DateTime<Utc>) -> Self;

    fn text(&self) -> &str;
    fn is_send_by_mailer(&self) -> bool;
    fn date(&self) -> DateTime<Utc>;

    fn set_text(&mut self, text : &str) -> ();
    fn set_is_send_by_mailer(&mut self, value : bool) -> ();
    fn set_date(&mut self, date : DateTime<Utc>) -> ();

    fn is_send_by_recipient(&self) -> bool { !self.is_send_by_mailer() }

}