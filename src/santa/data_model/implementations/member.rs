use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::{IAccountRelated, ILocalObject};
use crate::santa::data_model::traits::{IMember, IPoolRelated, IRoomRelated};

#[derive(Serialize, Deserialize, Clone)]
pub struct Member {
    id : String,
    account_id : String,
    room_id : String,
    pool_id : String,
    wishlist : String
}

impl ILocalObject for Member {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IAccountRelated for Member {
    fn account_id(&self) -> &str { self.account_id.as_str() }

    fn set_account_id(&mut self, account_id : &str) -> () { self.account_id = String::from(account_id); }
}

impl IRoomRelated for Member {
    fn room_id(&self) -> &str { self.room_id.as_str() }

    fn set_room_id(&mut self, room_id : &str) -> () {  self.room_id = String::from(room_id); }
}

impl IPoolRelated for Member {
    fn pool_id(&self) -> &str { self.pool_id.as_str() }

    fn set_pool_id(&mut self, pool_id : &str) -> () { self.pool_id = String::from(pool_id); }
}

impl IMember for Member {
    fn new(id : &str, account_id : &str, room_id : &str, pool_id : &str, wishlist : &str) -> Self {
        return Member {
            id: String::from(id),
            account_id: String::from(account_id),
            room_id: String::from(room_id),
            pool_id: String::from(pool_id),
            wishlist: String::from(wishlist),
        };
    }

    fn wishlist(&self) -> &str { self.wishlist.as_str() }

    fn set_wishlist(&mut self, wishlist : &str) -> () { self.wishlist = String::from(wishlist); }
}