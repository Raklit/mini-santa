use crate::core::data_model::traits::{IAccountRelated, ILocalObject, IPublicUserInfo};

pub struct PublicUserInfo {
    id : String,
    account_id : String,
    nickname : String,
    info : String
}

impl ILocalObject for PublicUserInfo {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IAccountRelated for PublicUserInfo {
    fn account_id(&self) -> &str { &self.account_id.as_str() }

    fn set_account_id(&mut self, account_id : &str) -> () { self.account_id = String::from(account_id) }
}

impl IPublicUserInfo for PublicUserInfo {
    fn nickname(&self) -> &str { self.nickname.as_str() }

    fn info(&self) -> &str { self.info.as_str() }

    fn set_nickname(&mut self, nickname : &str) -> () { self.nickname = String::from(nickname) }

    fn set_info(&mut self, info : &str) -> () { self.info = String::from(info) }
}