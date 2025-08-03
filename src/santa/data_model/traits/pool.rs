use chrono::{DateTime, Utc};

use crate::{core::data_model::traits::IAccountRelated, santa::data_model::enums::PoolState};

pub trait IPool : IAccountRelated {

    fn new(id : &str, name : &str, description : &str, creator_id : &str, min_price : u64, max_price : u64, creation_date : DateTime<Utc>, lifetime : u64, state : PoolState) -> Self;

    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn min_price(&self) -> u64;
    fn max_price(&self) -> u64;
    fn creation_date(&self) -> DateTime<Utc>;
    fn lifetime(&self) -> u64;
    fn state(&self) -> PoolState;

    fn set_name(&mut self, name : &str) -> ();
    fn set_description(&mut self, description : &str) -> ();
    fn set_min_price(&mut self, min_price : u64) -> ();
    fn set_max_price(&mut self, max_price : u64) -> ();
    fn set_creation_date(&mut self, creation_date : DateTime<Utc>) -> ();
    fn set_lifetime(&mut self, lifetime : u64) -> ();
    fn set_state(&mut self, state : PoolState) -> ();
}