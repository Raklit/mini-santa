use chrono::prelude::*;

use super::IAccountRelated;

pub trait IAccountSession : IAccountRelated {
    fn auth_token(&self) -> &str;

    fn refresh_token(&self) -> &str;

    fn auth_token_creation_date(&self) -> DateTime<Utc>;

    fn refresh_token_creation_date(&self) -> DateTime<Utc>;

    fn start_date(&self) -> DateTime<Utc>;

    fn last_usage_date(&self) -> DateTime<Utc>;

    fn set_auth_token(&mut self, auth_token : &str) -> ();

    fn set_refresh_token(&mut self, refresh_token : &str) -> ();

    fn set_auth_token_creation_date(&mut self, auth_token_creation_date : DateTime<Utc>) -> ();

    fn set_refresh_token_creation_date(&mut self, refresh_token_creation_date : DateTime<Utc>) -> ();

    fn set_start_date(&mut self, start_date : DateTime<Utc>) -> ();

    fn set_last_usage_date(&mut self, last_usage_date : DateTime<Utc>) -> ();
}