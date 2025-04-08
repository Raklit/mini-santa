use crate::core::data_model::traits::ILocalObject;

pub struct LocalObject {
    id : String
}

impl ILocalObject for LocalObject {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}