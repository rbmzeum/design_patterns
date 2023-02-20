use std::{rc::Rc, cell::RefCell};
use crate::patterns::behavioral::mediator::abstract_object::AbstractObject;
use crate::patterns::behavioral::mediator::object_manager::ObjectManager;

pub struct Exchange {
    manager: Rc<RefCell<ObjectManager>>,
    share: f64,
}

impl Exchange {

    pub fn new(manager: &Rc<RefCell<ObjectManager>>) -> Self {
        Self {
            manager: Rc::clone(manager),
            share: 0.0,
        }
    }

    pub fn buy_request(&mut self, share: f64) -> String {
        println!("Buy request");
        self.share = share;
        "Accept".to_string()
    }

    pub fn check_status(&self) {
        println!("Status");
        let status = "Success".to_string();
        let commission = 0.014;
        self.manager.borrow_mut().update_status(status, self.share + commission);
    }
    
}

impl AbstractObject for Exchange {

    fn get_manager(&self) -> Rc<RefCell<ObjectManager>> {
        Rc::clone(&self.manager)
    }

}