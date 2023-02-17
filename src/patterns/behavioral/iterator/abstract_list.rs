use std::{rc::Rc, cell::RefCell};

use super::{abstract_iterator::AbstractIterator, abstract_item::AbstractItem, item_type::ItemType};

pub trait AbstractList {
    fn create_iterator(&mut self, item_type: ItemType) -> Box<dyn AbstractIterator>;
    fn count(&self) -> usize;
    fn get(&mut self, index: usize) -> Option<&mut Rc<RefCell<dyn AbstractItem>>>;
    fn append(&mut self, item: Rc<RefCell<dyn AbstractItem>>);
    fn remove(&mut self, index: usize);
    fn clear(&mut self);
}