use crate::patterns::creational::prototype::abstract_portfolio::AbstractPortfolio;

#[derive(Clone)]
pub struct CoinbasePortfolio {
    id: u64,
}

impl CoinbasePortfolio {
    pub fn new() -> Self {
        Self {
            id: 0
        }
    }
}

impl AbstractPortfolio for CoinbasePortfolio {
    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_name(&self) -> String {
        format!("Coinbase portfolio{}", self.id)
    }
}