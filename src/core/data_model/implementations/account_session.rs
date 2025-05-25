use std::any::Any;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::{IAccountSession, IAccountRelated, ILocalObject};

#[derive(Serialize, Deserialize)]
pub struct AccountSession {
    pub id : String,
    pub account_id : String,
    pub auth_token : String,
    pub refresh_token : String,
    pub start_date : DateTime<Utc>,
    pub auth_token_creation_date : DateTime<Utc>,
    pub refresh_token_creation_date : DateTime<Utc>,
    pub last_usage_date : DateTime<Utc>
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
    fn as_any(&self) -> &dyn Any { return self; }

    fn auth_token(&self) -> &str { self.auth_token.as_str() }

    fn refresh_token(&self) -> &str { self.refresh_token.as_str() }

    fn auth_token_creation_date(&self) -> DateTime<Utc> { self.auth_token_creation_date }

    fn refresh_token_creation_date(&self) -> DateTime<Utc> { self.refresh_token_creation_date }

    fn start_date(&self) -> DateTime<Utc> { self.start_date }

    fn last_usage_date(&self) -> chrono::DateTime<chrono::Utc> { self.last_usage_date }

    fn set_auth_token(&mut self, auth_token : &str) -> () { self.auth_token = String::from(auth_token) }

    fn set_refresh_token(&mut self, refresh_token : &str) -> () { self.refresh_token = String::from(refresh_token) }

    fn set_auth_token_creation_date(&mut self, auth_token_creation_date : DateTime<Utc>) -> () { self.auth_token_creation_date = auth_token_creation_date }

    fn set_refresh_token_creation_date(&mut self, refresh_token_creation_date : DateTime<Utc>) -> () { self.refresh_token_creation_date = refresh_token_creation_date }

    fn set_start_date(&mut self, start_date : DateTime<Utc>) -> () { self.start_date = start_date }

    fn set_last_usage_date(&mut self, last_usage_date : DateTime<Utc>) -> () { self.last_usage_date = last_usage_date }
}