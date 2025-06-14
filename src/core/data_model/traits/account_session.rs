use chrono::prelude::*;

use super::IAccountRelated;

pub trait IAccountSession : IAccountRelated {

    fn new(id : &str, account_id : &str, access_token : &str, refresh_token : &str, start_date : DateTime<Utc>, access_token_creation_date : DateTime<Utc>, refresh_token_creation_date : DateTime<Utc>, last_usage_date : DateTime<Utc>) -> Self;

    fn access_token(&self) -> &str;

    fn refresh_token(&self) -> &str;

    fn access_token_creation_date(&self) -> DateTime<Utc>;

    fn refresh_token_creation_date(&self) -> DateTime<Utc>;

    fn start_date(&self) -> DateTime<Utc>;

    fn last_usage_date(&self) -> DateTime<Utc>;

    fn set_access_token(&mut self, access_token : &str) -> ();

    fn set_refresh_token(&mut self, refresh_token : &str) -> ();

    fn set_access_token_creation_date(&mut self, access_token_creation_date : DateTime<Utc>) -> ();

    fn set_refresh_token_creation_date(&mut self, refresh_token_creation_date : DateTime<Utc>) -> ();

    fn set_start_date(&mut self, start_date : DateTime<Utc>) -> ();

    fn set_last_usage_date(&mut self, last_usage_date : DateTime<Utc>) -> ();
}