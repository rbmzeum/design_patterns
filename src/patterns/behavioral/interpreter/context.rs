use super::price::Price;

pub struct Context {
    prices: Vec<Price>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            prices: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.prices.len()
    }

    pub fn lookup(&self, index: usize) -> &Price {
        &self.prices[index]
    }

    pub fn assign(&mut self, prices: Vec<Price>) {
        self.prices = prices;
    }
}