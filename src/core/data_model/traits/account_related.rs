use super::ILocalObject;

pub trait IAccountRelated : ILocalObject {
    fn account_id(&self) -> &str;
    
    fn set_account_id(&mut self, account_id : &str) -> ();
}