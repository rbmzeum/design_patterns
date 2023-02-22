use std::{rc::Rc, cell::RefCell};

use super::{abstract_observer::AbstractObserver, shares::Shares};

pub struct Portfolio {
    shares: Rc<RefCell<Shares>>,
}

impl Portfolio {

    pub fn new() -> Self {
        Self {
            shares: Rc::new(RefCell::new(Shares::new()))
        }
    }

    pub fn show(&self) {
        let usd = (*self.shares).borrow_mut().get_usd();
        let btc = (*self.shares).borrow_mut().get_btc();
        println!("Show (usd: {}, btc: {})", usd, btc);
    }
}

impl AbstractObserver for Portfolio {
    fn update(&self, shares: &Rc<RefCell<Shares>>) {

        let bot_usd = (*shares).borrow_mut().get_usd();
        let bot_btc = (*shares).borrow_mut().get_btc();

        (*self.shares).borrow_mut().set_usd(bot_usd);
        (*self.shares).borrow_mut().set_btc(bot_btc);

        println!("Update");
    }
}