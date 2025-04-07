use chrono::prelude::*;

use super::ILocalObject;

pub trait IMessage : ILocalObject {
    fn text(&self) -> &str;
    fn room_id(&self) -> &str;
    fn is_send_by_mailer(&self) -> bool;
    fn message_date(&self) -> DateTime<Utc>;

    fn set_text(&mut self, text : &str) -> ();
    fn set_room_id(&mut self, room_id : &str) -> ();
    fn set_is_send_by_mailer(&mut self, value : bool) -> ();
    fn set_message_date(&mut self, message_date : &DateTime<Utc>) -> ();

    fn is_send_by_recipient(&self) -> bool { !self.is_send_by_mailer() }

}