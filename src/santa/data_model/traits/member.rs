use crate::core::data_model::traits::IAccountRelated;
use crate::santa::data_model::traits::IRoomRelated;

pub trait IMember : IAccountRelated + IRoomRelated  {
    fn new(id : &str, account_id : &str, room_id : &str, wishlist : &str) -> Self;
    fn wishlist(&self) -> &str;
    fn set_wishlist(&mut self, wishlist : &str) -> ();
}