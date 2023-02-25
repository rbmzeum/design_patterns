use crate::patterns::behavioral::visitor::{abstract_visitor::AbstractVisitor, elements::{stock::Stock, portfolio::Portfolio}};

pub struct Buy {}

impl AbstractVisitor for Buy {

    fn visit_stock(&mut self, stock: &Stock) {
        println!("Buy stock {}", stock.get_name());
    }

    fn visit_portfolio(&mut self, portfolio: &Portfolio) {
        println!("Buy portfolio {}", portfolio.get_name());
    }

}