use crate::core::data_model::traits::{IAccountRelated, ILocalObject, IRecoveryUserInfo};

pub struct RecoveryUserInfo {
    id : String,
    account_id : String,
    email : String,
    phone : String,
}

impl ILocalObject for RecoveryUserInfo {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }

}

impl IAccountRelated for RecoveryUserInfo {
    fn account_id(&self) -> &str { &self.account_id.as_str() }

    fn set_account_id(&mut self, account_id : &str) -> () { self.account_id = String::from(account_id) }
}

impl IRecoveryUserInfo for RecoveryUserInfo {

    fn new(id : &str, account_id : &str, email : &str, phone : &str) -> Self {
        return RecoveryUserInfo {
            id: String::from(id),
            account_id: String::from(account_id),
            email: String::from(email),
            phone: String::from(phone),
        };
    }

    fn email(&self) -> &str { self.email.as_str() }

    fn phone(&self) -> &str { self.phone.as_str() }

    fn set_email(&mut self, email : &str) -> () { self.email = String::from(email) }

    fn set_phone(&mut self, phone : &str) -> () { self.phone = String::from(phone) }

}