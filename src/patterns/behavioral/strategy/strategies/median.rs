use crate::patterns::behavioral::strategy::abstract_strategy::AbstractStrategy;

pub struct Median {}

impl Median {
    pub fn new() -> Self {
        Self {}
    }
}

impl AbstractStrategy for Median {

    fn calculate(&self, prices: &Vec<f64>) -> f64 {
        if prices.len() == 0 {
            panic!("Prices should not be empty");
        }
        let mut sorted_prices = prices.clone();
        sorted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted_prices[sorted_prices.len() / 2]
    }
}