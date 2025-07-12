use crate::core::data_model::traits::ILocalObject;

pub trait IRoomRelated : ILocalObject {
    fn room_id(&self) -> &str;
    
    fn set_room_id(&mut self, room_id : &str) -> ();
}