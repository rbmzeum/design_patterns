use binance::market::Market;
use binance::api::Binance;

// use binance::api::*;
// use binance::model::*;
// use binance::market::*;

use crate::patterns::creational::abstract_factory::exchanges::abstract_market::AbstractMarket;

pub struct BinanceMarket {
    market: Market,
}

impl AbstractMarket for BinanceMarket {

    fn new() -> Self {
        Self {
            market: Binance::new(None, None)
        }
    }

    fn get_pairs_count(&self) -> usize {
        match self.market.get_all_book_tickers() {
            Ok(binance::model::BookTickers::AllBookTickers(answer)) => {
                answer.len()
            },
            Err(e) => {
                panic!("Error: {:?}", e)
            },
        }
    }

}