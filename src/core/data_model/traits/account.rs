use super::ILocalObject;

pub trait IAccount : ILocalObject {
    fn new(id : &str, login : &str, password_hash : &str, passwrod_salt : &str) -> Self;

    fn login(&self) -> &str;

    fn password_hash(&self) -> &str;
    
    fn passwrod_salt(&self) -> &str;

    fn set_login(&mut self, login : &str) -> ();
    
    fn set_password_hash(&mut self, password_hash : &str) -> ();
    
    fn set_password_salt(&mut self, passwrod_salt : &str) -> ();
}