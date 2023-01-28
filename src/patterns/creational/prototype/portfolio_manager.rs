use crate::patterns::creational::prototype::{
    abstract_portfolio::AbstractPortfolio,
    portfolios::binance_portfolio::BinancePortfolio,
    portfolios::coinbase_portfolio::CoinbasePortfolio,
};

pub struct PortfolioManager {
    potrfolios: Vec<Box<dyn AbstractPortfolio>>,

    binance_portfolio_prototype: BinancePortfolio,
    coinbase_portfolio_prototype: CoinbasePortfolio,
}

impl PortfolioManager {
    pub fn new() -> Self {
        let binance_portfolio_prototype = BinancePortfolio::new();
        let coinbase_portfolio_prototype = CoinbasePortfolio::new();

        Self {
            potrfolios: vec![],
            binance_portfolio_prototype: binance_portfolio_prototype,
            coinbase_portfolio_prototype: coinbase_portfolio_prototype,
        }
    }

    pub fn create_objects(&mut self) {
        let mut p = self.binance_portfolio_prototype.clone();
        p.set_id(2);
        self.potrfolios.push(Box::new(p));

        let mut p = self.binance_portfolio_prototype.clone();
        p.set_id(3);
        self.potrfolios.push(Box::new(p));

        let mut p = self.coinbase_portfolio_prototype.clone();
        p.set_id(4);
        self.potrfolios.push(Box::new(p));

        let mut p = self.coinbase_portfolio_prototype.clone();
        p.set_id(8);
        self.potrfolios.push(Box::new(p));
    }

    pub fn output(&self) {
        for p in &(self.potrfolios) {
            println!("{}", p.get_name());
        }
    }
}