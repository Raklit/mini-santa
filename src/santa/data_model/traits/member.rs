use crate::core::data_model::traits::IAccountRelated;
use crate::santa::data_model::traits::{IPoolRelated, IRoomRelated};

pub trait IMember : IAccountRelated + IRoomRelated  + IPoolRelated {
    fn new(id : &str, account_id : &str, room_id : &str, pool_id : &str, wishlist : &str) -> Self;
    fn wishlist(&self) -> &str;
    fn set_wishlist(&mut self, wishlist : &str) -> ();
}