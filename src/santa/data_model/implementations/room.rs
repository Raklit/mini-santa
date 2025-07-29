use serde::{Deserialize, Serialize};

use crate::core::data_model::traits::ILocalObject;
use crate::santa::data_model::enums::RoomState;
use crate::santa::data_model::traits::{IPoolRelated, IRoom};

#[derive(Serialize, Deserialize, Clone)]
pub struct Room {
    id : String,
    pool_id : String,
    mailer_id : String,
    recipient_id : String,
    room_state : RoomState
}

impl ILocalObject for Room {
    fn id(&self) -> &str { self.id.as_str() }

    fn set_id(&mut self, id : &str) -> () { self.id = String::from(id) }
}

impl IPoolRelated for Room {
    
    fn pool_id(&self) -> &str { self.pool_id.as_str() }

    fn set_pool_id(&mut self, pool_id : &str) -> () { self.pool_id = String::from(pool_id); }

}

impl IRoom for Room {

    fn new(id : &str, pool_id : &str, mailer_id : &str, recipient_id : &str, room_state : RoomState) -> Self {
        return Room { 
            id: String::from(id), 
            pool_id: String::from(pool_id), 
            mailer_id: String::from(mailer_id), 
            recipient_id: String::from(recipient_id),
            room_state: room_state, 
        }
    }

    fn mailer_id(&self) -> &str { self.mailer_id.as_str() }

    fn recipient_id(&self) -> &str { self.recipient_id.as_str() }

    fn room_state(&self) -> RoomState { self.room_state.clone() }

    fn set_mailer_id(&mut self, mailer_id : &str) -> () { self.mailer_id = String::from(mailer_id); }

    fn set_recipient_id(&mut self, recipient_id : &str) -> () { self.recipient_id = String::from(recipient_id); }

    fn set_room_state(&mut self, room_state : RoomState) -> () { self.room_state = room_state; }

}