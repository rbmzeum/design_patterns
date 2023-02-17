use std::{rc::Rc, cell::RefCell};

use super::abstract_item::AbstractItem;

pub trait AbstractIterator {
    fn first(&mut self);
    fn next(&mut self);
    fn is_done(&mut self) -> bool;
    fn current_item(&mut self) -> Option<Rc<RefCell<dyn AbstractItem>>>;
}