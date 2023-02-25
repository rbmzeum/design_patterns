use std::{rc::Rc, cell::RefCell};

use crate::patterns::behavioral::visitor::{abstract_element::AbstractElement, abstract_visitor::AbstractVisitor};

use super::stock::Stock;

pub struct Portfolio {
    stocks: Vec<Rc<RefCell<Stock>>>,
}

impl Portfolio {

    pub fn new(stocks: Vec<Rc<RefCell<Stock>>>) -> Self {
        Self {
            stocks,
        }
    }

    pub fn get_name(&self) -> String {
        let mut name = "".to_string();
        for (i, s) in self.stocks.iter().enumerate() {
            if i == 0 {
                name = format!("[ {}", (*s).borrow_mut().get_name());
            } else {
                name = format!("{}, {}", name, (*s).borrow_mut().get_name());
            }
        }
        format!("{} ]", name)
    }

}

impl AbstractElement for Portfolio {

    fn accept(&mut self, visitor: Rc<RefCell<dyn AbstractVisitor>>) {
        for stock in &mut self.stocks {
            (*stock).borrow_mut().accept(visitor.clone());
        }
        (*visitor).borrow_mut().visit_portfolio(self);
    }

}