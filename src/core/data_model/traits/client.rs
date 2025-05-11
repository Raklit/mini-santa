use super::ILocalObject;

pub trait IClient : ILocalObject {
    fn client_name(&self) -> &str;
    fn password_hash(&self) -> &str;
    fn passwrod_salt(&self) -> &str;

    fn set_client_name(&mut self, client_name : &str) -> ();
    fn set_password_hash(&mut self, password_hash : &str) -> ();
    fn set_password_salt(&mut self, passwrod_salt : &str) -> ();
}