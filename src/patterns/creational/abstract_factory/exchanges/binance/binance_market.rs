use binance::market::Market;
use binance::api::Binance;

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

}