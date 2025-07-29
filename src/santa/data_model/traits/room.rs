use crate::{core::data_model::traits::ILocalObject, santa::data_model::{enums::RoomState, traits::IPoolRelated}};

pub trait IRoom : ILocalObject + IPoolRelated {
    fn new(id : &str, pool_id : &str, mailer_id : &str, recipient_id : &str, room_state : RoomState) -> Self;
    
    fn mailer_id(&self) -> &str;
    fn recipient_id(&self) -> &str;
    fn room_state(&self) -> RoomState;
    fn set_mailer_id(&mut self, mailer_id : &str) -> ();
    fn set_recipient_id(&mut self, recipient_id : &str) -> ();
    fn set_room_state(&mut self, room_state : RoomState) -> ();
}