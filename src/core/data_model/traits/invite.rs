use crate::core::data_model::traits::ILocalObject;

pub trait IInvite : ILocalObject {
    fn new(id : &str, invite_code : &str, one_use : bool) -> Self;
    fn invite_code(&self) -> &str;
    fn set_invite_code(&mut self, invite_code : &str) -> ();
    fn one_use(&self) -> bool;
    fn set_one_use(&mut self, one_use : bool) -> ();
}