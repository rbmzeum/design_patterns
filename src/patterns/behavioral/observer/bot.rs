use super::abstract_observer::AbstractObserver;
use super::abstract_subject::AbstractSubject;
use super::shares::Shares;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Bot<'a, T: AbstractObserver> {
    shares: Rc<RefCell<Shares>>,
    observers: Vec<&'a T>,
}

impl<'a, T: AbstractObserver> Bot<'a, T> {

    pub fn new() -> Self {
        Self {
            shares: Rc::new(RefCell::new(Shares::new())),
            observers: vec![],
        }
    }

    pub fn set_usd(&mut self, count: f64) {
        (*self.shares).borrow_mut().set_usd(count);
        self.notify();
    }

    pub fn set_btc(&mut self, count: f64) {
        (*self.shares).borrow_mut().set_btc(count);
        self.notify();
    }

}

impl<'a, T: AbstractObserver> AbstractSubject<'a, T> for Bot<'a, T> {

    fn attach(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }

    fn detach(&mut self, index: usize) {
        if index < self.observers.len() {
            self.observers.remove(index);
        }
    }

    fn notify(&self) {
        for item in &self.observers {
            item.update(&self.shares);
        }
    }

}