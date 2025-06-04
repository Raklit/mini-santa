use chrono::{DateTime, Utc};

use crate::core::data_model::traits::{IAccountRelated, ILocalObject, IAuthCode};

pub struct AuthCode {
    id : String,
    account_id : String,
    code : String,
    creation_date: DateTime<Utc>
}

impl ILocalObject for AuthCode {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IAccountRelated for AuthCode {
    fn account_id(&self) -> &str { &self.account_id.as_str() }

    fn set_account_id(&mut self, account_id : &str) -> () { self.account_id = String::from(account_id) }
}

impl IAuthCode for AuthCode {
    fn code(&self) -> &str { self.code.as_str() }

    fn creation_date(&self) -> DateTime<Utc> { self.creation_date }

    fn set_code(&mut self, code : &str) -> () { self.code = String::from(code) }

    fn set_creation_date(&mut self, creation_date : DateTime<Utc>) -> () { self.creation_date = creation_date }
}