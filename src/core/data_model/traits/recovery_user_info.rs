use super::IAccountRelated;

pub trait IRecoveryUserInfo : IAccountRelated {

    fn new(id : &str, account_id : &str, email : &str, phone : &str) -> Self;

    fn email(&self) -> &str;

    fn phone(&self) -> &str;

    fn set_email(&mut self, email : &str) -> ();

    fn set_phone(&mut self, phone : &str) -> ();
}