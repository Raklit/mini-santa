use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::{ILocalObject, IInvite};

#[derive(Serialize, Deserialize, Clone)]
pub struct Invite {
    id : String,
    invite_code : String,
    one_use : bool
}

impl ILocalObject for Invite {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}


impl IInvite for Invite {
    fn new(id : &str, invite_code : &str, one_use : bool) -> Self {
        return Invite {
            id: String::from(id), 
            invite_code: String::from(invite_code), 
            one_use: one_use
        };
    }

    fn invite_code(&self) -> &str { self.invite_code.as_str() }

    fn set_invite_code(&mut self, invite_code : &str) -> () { self.invite_code = String::from(invite_code); }

    fn one_use(&self) -> bool { self.one_use }

    fn set_one_use(&mut self, one_use : bool) -> () { self.one_use = one_use; }
}