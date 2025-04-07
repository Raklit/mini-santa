pub trait ILocalObject {
    fn id(&self) -> &str;

    fn set_id(&mut self, id : &str) -> ();
}