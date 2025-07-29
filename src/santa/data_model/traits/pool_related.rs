use crate::core::data_model::traits::ILocalObject;

pub trait IPoolRelated : ILocalObject {
    fn pool_id(&self) -> &str;
    
    fn set_pool_id(&mut self, pool_id : &str) -> ();
}