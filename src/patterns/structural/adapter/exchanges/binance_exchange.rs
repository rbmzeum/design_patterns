use binance::market::Market;
use binance::api::Binance;

use crate::patterns::structural::adapter::abstract_exchange::AbstractExchange;

pub struct BinanceExchange {
    market: Market, // improve to AbstractMarket -> BinanceMarket
}

impl BinanceExchange {
    pub fn new() -> Self {
        Self {
            market: Binance::new(None, None)
        }
    }
}

impl AbstractExchange for BinanceExchange {
    fn get_pairs_count(&self) -> usize {
        // BinanceMarket::get_pairs_count()
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