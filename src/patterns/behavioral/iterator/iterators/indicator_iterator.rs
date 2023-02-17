use std::{rc::Rc, cell::RefCell};

use crate::patterns::behavioral::iterator::{abstract_iterator::AbstractIterator, abstract_item::AbstractItem, item_type::ItemType, abstract_list::AbstractList};

pub struct IndicatorIterator {
    list: Rc<RefCell<dyn AbstractList>>,
    current: usize,
}

impl IndicatorIterator {

    pub fn new(list: Rc<RefCell<dyn AbstractList>>) -> Self {
        Self {
            list,
            current: 0,
        }
    }

}

impl AbstractIterator for IndicatorIterator {

    fn first(&mut self) {
        if self.list.borrow_mut().count() > 0 {
            self.current = 0;
        }
        match &self.current_item() {
            Some(item) => {
                if item.borrow_mut().get_type() != ItemType::INDICATOR {
                    self.next();
                }
            },
            None => {},
        }
    }

    fn next(&mut self) {
        let start = self.current.clone() + 1;
        let end = self.list.borrow_mut().count() + 1;
        if start < end {
            for i in start..end {
                self.current = i;
                match &self.current_item() {
                    Some(item) => {
                        if item.borrow_mut().get_type() == ItemType::INDICATOR {
                            break;
                        }
                    },
                    None => {
                        break;
                    },
                }
            }
        }
    }

    fn is_done(&mut self) -> bool {
        let save_current = self.current.clone();
        let start = self.current.clone();
        let end = self.list.borrow_mut().count();

        if start < end {
            for i in start..end {
                self.current = i;
                match &self.current_item() {
                    Some(item) => {
                        if item.borrow_mut().get_type() == ItemType::INDICATOR {
                            self.current = save_current;
                            return false;
                        }
                    },
                    None => {
                        // Nope
                    },
                }
            }
            self.current = save_current;

            true
        } else {
            true
        }
    }

    fn current_item(&mut self) -> Option<Rc<RefCell<dyn AbstractItem>>> {
        if self.list.borrow_mut().count() == 0 {
            None
        } else {
            match self.list.borrow_mut().get(self.current) {
                Some(item) => {
                    if item.borrow_mut().get_type() == ItemType::INDICATOR {
                        Some(Rc::clone(item))
                    } else {
                        None
                    }
                },
                None => { None },
            }
        }
    }

}