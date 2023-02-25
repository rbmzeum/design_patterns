use std::{rc::Rc, cell::RefCell};

use crate::patterns::behavioral::visitor::{abstract_element::AbstractElement, abstract_visitor::AbstractVisitor};

pub struct Stock {
    name: String,
}

impl Stock {

    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

}

impl AbstractElement for Stock {

    fn accept(&mut self, visitor: Rc<RefCell<dyn AbstractVisitor>>) {
        (*visitor).borrow_mut().visit_stock(self);
    }

}