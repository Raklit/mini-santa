use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::{IAccountRelated, ILocalObject};
use crate::santa::data_model::enums::PoolState;
use crate::santa::data_model::traits::IPool;

#[derive(Serialize, Deserialize, Clone)]
pub struct Pool {
    id : String,
    name : String,
    description : String,
    account_id : String,
    min_price : u64,
    max_price : u64,
    is_creator_involved : bool,
    creation_date : DateTime<Utc>,
    lifetime : u64,
    state : PoolState

}

impl ILocalObject for Pool {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IAccountRelated for Pool {
    fn account_id(&self) -> &str { self.account_id.as_str() }

    fn set_account_id(&mut self, account_id : &str) -> () { self.account_id = String::from(account_id) }
}

impl IPool for Pool {
    fn new(id : &str, name : &str, description : &str, account_id : &str, min_price : u64, max_price : u64, is_creator_involved : bool, creation_date : DateTime<Utc>, lifetime : u64, state : PoolState) -> Self {
        return Pool {
            id: String::from(id),
            name: String::from(name),
            description: String::from(description),
            account_id: String::from(account_id),
            min_price: min_price,
            max_price: max_price,
            is_creator_involved: is_creator_involved,
            creation_date: creation_date,
            lifetime: lifetime,
            state: state
        };
    }

    fn name(&self) -> &str { self.name.as_str() }

    fn description(&self) -> &str { self.description.as_str() }

    fn min_price(&self) -> u64 { self.min_price }

    fn max_price(&self) -> u64 { self.max_price }

    fn is_creator_involdved(&self) -> bool { self.is_creator_involved }

    fn creation_date(&self) -> DateTime<Utc> { self.creation_date }

    fn lifetime(&self) -> u64 { self.lifetime }

    fn state(&self) -> PoolState { self.state.clone() }

    fn set_name(&mut self, name : &str) -> () { self.name = String::from(name); }

    fn set_description(&mut self, description : &str) -> () { self.description = String::from(description); }

    fn set_min_price(&mut self, min_price : u64) -> () { self.min_price = min_price; }

    fn set_max_price(&mut self, max_price : u64) -> () { self.max_price = max_price; }

    fn set_is_creator_involved(&mut self, is_creator_involved : bool) -> () { self.is_creator_involved = is_creator_involved; }
    
    fn set_creation_date(&mut self, creation_date : DateTime<Utc>) -> () { self.creation_date = creation_date; }

    fn set_lifetime(&mut self, lifetime : u64) -> () { self.lifetime = lifetime; }

    fn set_state(&mut self, state : PoolState) -> () { self.state = state; }
}