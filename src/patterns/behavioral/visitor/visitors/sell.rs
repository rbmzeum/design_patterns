use crate::patterns::behavioral::visitor::{abstract_visitor::AbstractVisitor, elements::{stock::Stock, portfolio::Portfolio}};

pub struct Sell {}

impl AbstractVisitor for Sell {

    fn visit_stock(&mut self, stock: &Stock) {
        println!("Sell stock {}", stock.get_name());
    }

    fn visit_portfolio(&mut self, portfolio: &Portfolio) {
        println!("Sell portfolio {}", portfolio.get_name());
    }

}