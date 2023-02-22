use std::{rc::Rc, cell::RefCell};

use super::shares::Shares;

pub trait AbstractObserver {
    fn update(&self, shares: &Rc<RefCell<Shares>>);
}