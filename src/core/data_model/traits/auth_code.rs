use chrono::{DateTime, Utc};

use super::IAccountRelated;

pub trait IAuthCode : IAccountRelated {
    fn code(&self) -> &str;

    fn creation_date(&self) -> DateTime<Utc>;

    fn set_code(&mut self, code : &str) -> ();
    
    fn set_creation_date(&mut self, date : DateTime<Utc>) -> ();
}