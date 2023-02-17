use std::{rc::Rc, cell::RefCell, borrow::BorrowMut};

use super::{abstract_item::AbstractItem, abstract_list::AbstractList, abstract_iterator::AbstractIterator, item_type::ItemType, iterators::{indicator_iterator::IndicatorIterator, strategy_iterator::StrategyIterator}};

#[derive(Clone)]
pub struct Scripts {
    items: RefCell<Vec<Rc<RefCell<dyn AbstractItem>>>>,
}


impl Scripts {

    pub fn new() -> Self {
        Self {
            items: RefCell::new(vec![]),
        }
    }

}

impl AbstractList for Scripts {

    fn create_iterator(&mut self, item_type: ItemType) -> Box<dyn AbstractIterator> {
        match item_type {
            ItemType::INDICATOR => {
                Box::new(IndicatorIterator::new(Rc::new(RefCell::new(self.clone()))))
            },
            ItemType::STRATEGY => {
                Box::new(StrategyIterator::new(Rc::new(RefCell::new(self.clone()))))
            },
        }
    }

    fn count(&self) -> usize {
        self.items.borrow_mut().len()
    }

    fn get(&mut self, index: usize) -> Option<&mut Rc<RefCell<dyn AbstractItem>>> {
        if index >= self.items.get_mut().len() {
            None
        } else {
            Some(self.items.get_mut()[index].borrow_mut())
        }
    }

    fn append(&mut self, item: Rc<RefCell<dyn AbstractItem>>) {
        (*self.items.get_mut()).push(item);
    }

    fn remove(&mut self, index: usize) {
        (*self.items.get_mut()).remove(index);
    }

    fn clear(&mut self) {
        (*self.items.get_mut()).clear();
    }

}