use crate::core::data_model::traits::{ILocalObject, IRoom};

pub struct Room {
    id : String,
    mailer_id : String,
    recipient_id : String
}

impl ILocalObject for Room {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IRoom for Room {
    fn mailer_id(&self) -> &str { self.mailer_id.as_str() }

    fn recipient_id(&self) -> &str { self.recipient_id.as_str() }

    fn set_mailer_id(&mut self, mailer_id : &str) -> () { self.mailer_id = String::from(mailer_id) }

    fn set_recipient_id(&mut self, recipient_id : &str) -> () { self.recipient_id = String::from(recipient_id) }
}