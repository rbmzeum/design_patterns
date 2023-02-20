use std::{rc::Rc, cell::RefCell};
use crate::patterns::behavioral::mediator::abstract_object::AbstractObject;
use crate::patterns::behavioral::mediator::object_manager::ObjectManager;

pub struct Bot {
    manager: Rc<RefCell<ObjectManager>>,
}

impl Bot {

    pub fn new(manager: &Rc<RefCell<ObjectManager>>) -> Self {
        Self {
            manager: Rc::clone(manager),
        }
    }

    pub fn buy(&self) {
        self.manager.borrow_mut().buy();
    }

}

impl AbstractObject for Bot {

    fn get_manager(&self) -> Rc<RefCell<ObjectManager>> {
        Rc::clone(&self.manager)
    }

}