use super::IAccountRelated;

pub trait IPublicUserInfo : IAccountRelated {

    fn new(id : &str, account_id : &str, nickname : &str, info : &str) -> Self;

    fn nickname(&self) -> &str;
    fn info(&self) -> &str;

    fn set_nickname(&mut self, nickname : &str) -> ();
    fn set_info(&mut self, info : &str) -> ();
}