use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::{IAccountRelated, ILocalObject, IPublicUserInfo};

#[derive(Serialize, Deserialize, Clone)]
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

    fn new(id : &str, account_id : &str, nickname : &str, info : &str) -> Self {
        return PublicUserInfo {
            id: String::from(id),
            account_id: String::from(account_id),
            nickname: String::from(nickname),
            info: String::from(info)
        };
    }

    fn nickname(&self) -> &str { self.nickname.as_str() }

    fn info(&self) -> &str { self.info.as_str() }

    fn set_nickname(&mut self, nickname : &str) -> () { self.nickname = String::from(nickname) }

    fn set_info(&mut self, info : &str) -> () { self.info = String::from(info) }
}