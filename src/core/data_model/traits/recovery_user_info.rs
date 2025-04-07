use super::IAccountRelated;

pub trait IRecoveryUserInfo : IAccountRelated {
    fn email(&self) -> &str;
    fn phone(&self) -> &str;
    fn telegram(&self) -> &str;

    fn set_email(&mut self, email : &str) -> ();
    fn set_phone(&mut self, phone : &str) -> ();
    fn set_telegram(&mut self, telegram : &str) -> ();
}