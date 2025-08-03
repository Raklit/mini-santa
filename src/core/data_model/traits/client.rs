use super::ILocalObject;

pub trait IClient : ILocalObject {

    fn new(id : &str, client_name : &str, password_hash : &str, passwrod_salt : &str, redirect_uri : &str, is_public : bool) -> Self;

    fn client_name(&self) -> &str;
    
    fn password_hash(&self) -> &str;
    
    fn password_salt(&self) -> &str;

    fn redirect_uri(&self) -> &str;

    fn is_public(&self) -> bool;

    fn set_client_name(&mut self, client_name : &str) -> ();
    
    fn set_password_hash(&mut self, password_hash : &str) -> ();
    
    fn set_password_salt(&mut self, passwrod_salt : &str) -> ();

    fn set_redirect_uri(&mut self, redirect_uri : &str) -> ();

    fn set_is_public(&mut self, is_public : bool) -> ();
}