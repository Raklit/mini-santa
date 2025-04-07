use super::ILocalObject;

pub trait IRoom : ILocalObject {
    fn mailer_id(&self) -> &str;
    fn recipient_id(&self) -> &str;

    fn set_mailer_id(&mut self, mailer_id : &str) -> ();
    fn set_recipient_id(&mut self, recipient_id : &str) -> ();
}