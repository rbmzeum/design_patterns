use std::{rc::Rc, cell::RefCell};

use super::objects::{
    bot::Bot,
    exchange::Exchange,
    history::History,
    portfolio::Portfolio,
};

pub struct ObjectManager {
    bot: Option<Rc<RefCell<Bot>>>,
    exchange: Option<Rc<RefCell<Exchange>>>,
    history: Option<Rc<RefCell<History>>>,
    portfolio: Option<Rc<RefCell<Portfolio>>>,
}

impl ObjectManager {

    pub fn new() -> Self {
        Self {
            bot: None,
            exchange: None,
            history: None,
            portfolio: None,
        }
    }

    pub fn set_bot(&mut self, bot: &Rc<RefCell<Bot>>) {
        self.bot = Some(Rc::clone(bot));
    }

    pub fn set_exchange(&mut self, exchange: &Rc<RefCell<Exchange>>) {
        self.exchange = Some(Rc::clone(exchange));
    }

    pub fn set_history(&mut self, history: &Rc<RefCell<History>>) {
        self.history = Some(Rc::clone(history));
    }

    pub fn set_portfolio(&mut self, portfolio: &Rc<RefCell<Portfolio>>) {
        self.portfolio = Some(Rc::clone(portfolio));
    }

    // Commands
    pub fn buy(&mut self) {
        let share = match &self.portfolio {
            Some(portfolio) => (*(*portfolio)).borrow_mut().get_share(),
            None => 0.0,
        };

        if share > 0.0 {
            let request_result = match &self.exchange {
                Some(exchange) => {
                    Some((*(*exchange)).borrow_mut().buy_request(share))
                },
                None => {
                    None
                },
            };

            match request_result {
                Some(rr) => {
                    match &self.history {
                        Some(history) => {
                            (*(*history)).borrow_mut().log(format!("{} {}", rr, share));
                        },
                        None => {},
                    }
                },
                None => {},
            };
        }
    }

    pub fn update_status(&mut self, status: String, share: f64) {
        match &self.portfolio {
            Some(portfolio) => {
                (*(*portfolio)).borrow_mut().spend(share);

                match &self.history {
                    Some(history) => {
                        (*(*history)).borrow_mut().log(format!("{} {}", status, share));
                    },
                    None => {},
                };
            },
            None => {},
        };
    }

}