use crate::patterns::creational::prototype::abstract_portfolio::AbstractPortfolio;

#[derive(Clone)]
pub struct BinancePortfolio {
    id: u64,
}

impl BinancePortfolio {
    pub fn new() -> Self {
        Self {
            id: 0
        }
    }
}

impl AbstractPortfolio for BinancePortfolio {
    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_name(&self) -> String {
        format!("Binance portfolio{}", self.id)
    }
}
