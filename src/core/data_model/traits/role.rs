use crate::core::data_model::traits::ILocalObject;

pub trait IRole : ILocalObject {

    fn new(id : &str, name : &str, tags : &str) -> Self;
    
    async fn name(&self) -> &str;

    async fn tags(&self) -> &str;

    async fn set_name(&mut self, name : &str) -> ();

    async fn set_tags(&mut self, tags : &str) -> ();

}