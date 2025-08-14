use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::{IAccountSession, IAccountRelated, ILocalObject};

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountSession {
    id : String,
    account_id : String,
    access_token : String,
    refresh_token : String,
    start_date : DateTime<Utc>,
    access_token_creation_date : DateTime<Utc>,
    refresh_token_creation_date : DateTime<Utc>,
    last_usage_date : DateTime<Utc>
}

impl ILocalObject for AccountSession {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IAccountRelated for AccountSession {
    fn account_id(&self) -> &str { &self.account_id.as_str() }

    fn set_account_id(&mut self, account_id : &str) -> () { self.account_id = String::from(account_id) }
}

impl IAccountSession for AccountSession {

    fn new(id : &str, account_id : &str, access_token : &str, refresh_token : &str, start_date : DateTime<Utc>, access_token_creation_date : DateTime<Utc>, refresh_token_creation_date : DateTime<Utc>, last_usage_date : DateTime<Utc>) -> Self {
        return AccountSession {
            id: String::from(id),
            account_id: String::from(account_id),
            access_token: String::from(access_token),
            refresh_token: String::from(refresh_token),
            start_date: start_date,
            access_token_creation_date: access_token_creation_date,
            refresh_token_creation_date: refresh_token_creation_date,
            last_usage_date: last_usage_date,
        };
    }

    fn access_token(&self) -> &str { self.access_token.as_str() }

    fn refresh_token(&self) -> &str { self.refresh_token.as_str() }

    fn access_token_creation_date(&self) -> DateTime<Utc> { self.access_token_creation_date }

    fn refresh_token_creation_date(&self) -> DateTime<Utc> { self.refresh_token_creation_date }

    fn start_date(&self) -> DateTime<Utc> { self.start_date }

    fn last_usage_date(&self) -> chrono::DateTime<chrono::Utc> { self.last_usage_date }

    fn set_access_token(&mut self, access_token : &str) -> () { self.access_token = String::from(access_token) }

    fn set_refresh_token(&mut self, refresh_token : &str) -> () { self.refresh_token = String::from(refresh_token) }

    fn set_access_token_creation_date(&mut self, access_token_creation_date : DateTime<Utc>) -> () { self.access_token_creation_date = access_token_creation_date }

    fn set_refresh_token_creation_date(&mut self, refresh_token_creation_date : DateTime<Utc>) -> () { self.refresh_token_creation_date = refresh_token_creation_date }

    fn set_start_date(&mut self, start_date : DateTime<Utc>) -> () { self.start_date = start_date }

    fn set_last_usage_date(&mut self, last_usage_date : DateTime<Utc>) -> () { self.last_usage_date = last_usage_date }
}