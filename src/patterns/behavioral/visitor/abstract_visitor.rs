use super::elements::{stock::Stock, portfolio::Portfolio};

pub trait AbstractVisitor {
    fn visit_stock(&mut self, stock: &Stock);
    fn visit_portfolio(&mut self, portfolio: &Portfolio);
}