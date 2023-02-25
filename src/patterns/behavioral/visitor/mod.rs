pub mod visitors;
pub mod elements;
pub mod abstract_element;
pub mod abstract_visitor;

// Example
pub fn behavioral_visitor() {
    use std::{rc::Rc, cell::RefCell};
    use self::{
        elements::{
            stock::Stock,
            portfolio::Portfolio
        },
        abstract_element::AbstractElement,
        abstract_visitor::AbstractVisitor,
        visitors::{
            buy::Buy,
            sell::Sell
        }
    };

    let s1 = Stock::new("ASSET1".to_string());
    let s2 = Stock::new("ASSET2".to_string());
    let s3 = Stock::new("ASSET3".to_string());
    let s4 = Stock::new("ASSET4".to_string());

    let p = Portfolio::new(vec![
        Rc::new(RefCell::new(s1)),
        Rc::new(RefCell::new(s2)),
        Rc::new(RefCell::new(s3)),
    ]);

    let v1 = Buy{};
    let v2 = Sell{};

    let elements: Vec<Box<dyn AbstractElement>> = vec![Box::new(s4), Box::new(p)];
    let visitors: Vec<Rc<RefCell<dyn AbstractVisitor>>> = vec![Rc::new(RefCell::new(v1)), Rc::new(RefCell::new(v2))];

    for mut e in elements {
        for v in &visitors {
            e.accept(v.clone());
        }
    }
}