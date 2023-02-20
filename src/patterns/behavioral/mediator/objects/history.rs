use std::{rc::Rc, cell::RefCell};
use crate::patterns::behavioral::mediator::abstract_object::AbstractObject;
use crate::patterns::behavioral::mediator::object_manager::ObjectManager;

pub struct History {
    manager: Rc<RefCell<ObjectManager>>,
    log: Vec<String>,
}

impl History {

    pub fn new(manager: &Rc<RefCell<ObjectManager>>) -> Self {
        Self {
            manager: Rc::clone(manager),
            log: vec![],
        }
    }

    pub fn log(&mut self, s: String) {
        self.log.push(s);
    }

    pub fn show_log(&self) {
        for s in &self.log {
            println!("Log: {}", s);
        }
    }
    
}

impl AbstractObject for History {

    fn get_manager(&self) -> Rc<RefCell<ObjectManager>> {
        Rc::clone(&self.manager)
    }

}