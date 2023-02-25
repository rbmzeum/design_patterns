use std::{rc::Rc, cell::RefCell};

use super::abstract_visitor::AbstractVisitor;

pub trait AbstractElement {
    fn accept(&mut self, visitor: Rc<RefCell<dyn AbstractVisitor>>);
}