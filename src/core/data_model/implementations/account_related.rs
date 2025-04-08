use crate::core::data_model::traits::{IAccountRelated, ILocalObject};

pub struct AccountRelated {
    id : String,
    account_id : String
}

impl ILocalObject for AccountRelated {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IAccountRelated for AccountRelated {
    fn account_id(&self) -> &str { self.account_id.as_str() }

    fn set_account_id(&mut self, account_id : &str) -> () { self.account_id = String::from(account_id) }
}