use std::{rc::Rc, cell::RefCell};

use crate::patterns::behavioral::iterator::{items::strategy::Strategy, item_type::ItemType};

pub mod item_type;
pub mod abstract_item;
pub mod abstract_iterator;
pub mod items;
pub mod abstract_list;
pub mod iterators;
pub mod scripts;

// Example
pub fn behavioral_iterator() {
    use scripts::Scripts;
    use abstract_list::AbstractList;
    use items::indicator::Indicator;

    let mut scripts = Scripts::new();

    let item0 = Rc::new(RefCell::new(Indicator::new("One".to_string())));
    let item1 = Rc::new(RefCell::new(Indicator::new("Two".to_string())));
    let item2 = Rc::new(RefCell::new(Strategy::new("Three".to_string())));
    let item3 = Rc::new(RefCell::new(Indicator::new("Four".to_string())));
    let item4 = Rc::new(RefCell::new(Strategy::new("Five".to_string())));
    let item5 = Rc::new(RefCell::new(Indicator::new("Six".to_string())));
    let item6 = Rc::new(RefCell::new(Strategy::new("Seven".to_string())));
    let item7 = Rc::new(RefCell::new(Strategy::new("Ate".to_string())));
    let item8 = Rc::new(RefCell::new(Indicator::new("Nine".to_string())));
    let item9 = Rc::new(RefCell::new(Indicator::new("Ten".to_string())));

    scripts.append(item0);
    scripts.append(item1);
    scripts.append(item2);
    scripts.append(item3);
    scripts.append(item4);
    scripts.append(item5);
    scripts.append(item6);
    scripts.append(item7);
    scripts.append(item8);
    scripts.append(item9);

    let mut indicator_iterator = scripts.create_iterator(ItemType::INDICATOR);
    indicator_iterator.first();
    while !indicator_iterator.is_done() {
        let current_indicator = indicator_iterator.current_item();
        match current_indicator {
            Some(item) => {
                (*item).borrow_mut().print_name();
                (*item).borrow_mut().rename("Visited indicator".to_string());
            },
            None => {},
        };
        indicator_iterator.next();
    }

    let mut strategy_iterator = scripts.create_iterator(ItemType::STRATEGY);
    strategy_iterator.first();
    while !strategy_iterator.is_done() {
        let current_indicator = strategy_iterator.current_item();
        match current_indicator {
            Some(item) => {
                (*item).borrow_mut().print_name();
                (*item).borrow_mut().rename("Visited strategy".to_string());
            },
            None => {},
        };
        strategy_iterator.next();
    }

}