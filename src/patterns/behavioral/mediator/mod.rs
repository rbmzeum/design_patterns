pub mod abstract_object;
pub mod object_manager;
pub mod objects;

// Example
pub fn behavioral_mediator() {
    use std::{rc::Rc, cell::RefCell};
    use object_manager::ObjectManager;
    use crate::patterns::behavioral::mediator::objects::{
        bot::Bot,
        exchange::Exchange,
        history::History,
        portfolio::Portfolio
    };

    // Init
    let manager = ObjectManager::new();
    let rc_manager = Rc::new(RefCell::new(manager));

    let bot = Bot::new(&Rc::clone(&rc_manager));
    let rc_bot = Rc::new(RefCell::new(bot));
    rc_manager.borrow_mut().set_bot(&rc_bot);

    let exchange = Exchange::new(&Rc::clone(&rc_manager));
    let rc_exchange = Rc::new(RefCell::new(exchange));
    rc_manager.borrow_mut().set_exchange(&rc_exchange);

    let history = History::new(&Rc::clone(&rc_manager));
    let rc_history = Rc::new(RefCell::new(history));
    rc_manager.borrow_mut().set_history(&rc_history);

    let portfolio = Portfolio::new(&Rc::clone(&rc_manager));
    let rc_portfolio = Rc::new(RefCell::new(portfolio));
    rc_manager.borrow_mut().set_portfolio(&rc_portfolio);

    // Run
    rc_portfolio.borrow_mut().show_balance();
    rc_exchange.borrow_mut().check_status();
    rc_bot.borrow_mut().buy();
    rc_exchange.borrow_mut().check_status();
    rc_portfolio.borrow_mut().show_balance();
    rc_history.borrow_mut().show_log();

}