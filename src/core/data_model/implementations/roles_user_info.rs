use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::{IAccountRelated, ILocalObject, IRolesUserInfo};

#[derive(Serialize, Deserialize, Clone)]
pub struct RolesUserInfo {
    id : String,
    account_id : String,
    role_id : String,
    params : String
}

impl IRolesUserInfo for RolesUserInfo {

    fn new(id : &str, account_id : &str, role_id : &str, params : &str) -> Self {
        return RolesUserInfo {
            id : String::from(id),
            account_id : String::from(account_id),
            role_id : String::from(role_id),
            params : String::from(params)
        };
    }

    fn role_id(&self) -> &str { self.role_id.as_str() }

    fn params(&self) -> &str { self.params.as_str() }
    
    fn set_role_id(&mut self, role_id : &str) -> () { self.role_id = String::from(role_id); }
    
    fn set_params(&mut self, params : &str) -> () { self.params = String::from(params); }
}

impl ILocalObject for RolesUserInfo {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IAccountRelated for RolesUserInfo {
    fn account_id(&self) -> &str { self.account_id.as_str() }

    fn set_account_id(&mut self, account_id : &str) -> () { self.account_id = String::from(account_id) }
}