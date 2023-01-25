use crate::patterns::creational::abstract_factory::exchanges::{
    abstract_exchange_factory::AbstractExchangeFactory,
    abstract_account::AbstractAccount,
    abstract_market::AbstractMarket,
};

use super::binance_market::BinanceMarket;
use super::binance_account::BinanceAccount;

#[derive(Debug)]
pub struct BinanceExchangeFactory;

impl AbstractExchangeFactory for BinanceExchangeFactory {

    fn create_market(&self) -> Box<dyn AbstractMarket> {
        let market = BinanceMarket::new();
        Box::new(market)
    }

    fn create_account(&self) -> Box<dyn AbstractAccount> {
        let account = BinanceAccount::new();
        Box::new(account)
    }

}

