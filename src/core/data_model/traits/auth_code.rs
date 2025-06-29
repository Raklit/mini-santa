use chrono::{DateTime, Utc};

use super::IAccountRelated;

pub trait IAuthCode : IAccountRelated {

    fn new(id : &str, account_id : &str, code : &str, creation_date : DateTime<Utc>) -> Self;

    fn code(&self) -> &str;

    fn creation_date(&self) -> DateTime<Utc>;

    fn set_code(&mut self, code : &str) -> ();
    
    fn set_creation_date(&mut self, date : DateTime<Utc>) -> ();
}