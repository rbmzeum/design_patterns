
use std::{rc::Rc, cell::RefCell};
use super::object_manager::ObjectManager;

pub trait AbstractObject {
    fn get_manager(&self) -> Rc<RefCell<ObjectManager>>;
}