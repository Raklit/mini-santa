use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::{ILocalObject, IRole};

#[derive(Serialize, Deserialize, Clone)]
pub struct Role {
    id : String,
    name : String,
    tags : String
}

impl IRole for Role {

    fn new(id : &str, name : &str, tags : &str) -> Self {
        return Role {
            id : String::from(id),
            name : String::from(name),
            tags : String::from(tags)
        };
    }

    async fn name(&self) -> &str { self.name.as_str() }

    async fn tags(&self) -> &str { self.tags.as_str() }

    async fn set_name(&mut self, name : &str) -> () { self.name = String::from(name) }

    async fn set_tags(&mut self, tags : &str) -> () { self.tags = String::from(tags) }
}

impl ILocalObject for Role {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}