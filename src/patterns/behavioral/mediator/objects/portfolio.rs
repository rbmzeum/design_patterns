use std::{rc::Rc, cell::RefCell};
use crate::patterns::behavioral::mediator::abstract_object::AbstractObject;
use crate::patterns::behavioral::mediator::object_manager::ObjectManager;

pub struct Portfolio {
    manager: Rc<RefCell<ObjectManager>>,
    balance: f64,
}

impl Portfolio {

    pub fn new(manager: &Rc<RefCell<ObjectManager>>) -> Self {
        Self {
            manager: Rc::clone(manager),
            balance: 1.0,
        }
    }

    pub fn get_share(&self) -> f64 {
        0.242
    }

    pub fn spend(&mut self, share: f64) {
        self.balance -= share;
    }

    pub fn show_balance(&self) {
        println!("Balance: {}", self.balance);
    }
    
}

impl AbstractObject for Portfolio {

    fn get_manager(&self) -> Rc<RefCell<ObjectManager>> {
        Rc::clone(&self.manager)
    }

}