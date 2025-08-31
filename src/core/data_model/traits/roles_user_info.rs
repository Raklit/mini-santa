use crate::core::data_model::traits::{IAccountRelated, ILocalObject};

pub trait IRolesUserInfo : ILocalObject + IAccountRelated {

    fn new(id : &str, account_id : &str, role_id : &str, params : &str) -> Self;
    
    fn role_id(&self) -> &str;
    
    fn params(&self) -> &str;

    fn set_role_id(&mut self, role_id : &str) -> ();

    fn set_params(&mut self, params : &str) -> ();
}