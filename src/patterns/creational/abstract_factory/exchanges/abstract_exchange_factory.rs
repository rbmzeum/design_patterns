use super::abstract_market::AbstractMarket;
use super::abstract_account::AbstractAccount;

pub trait AbstractExchangeFactory {
    fn create_market(&self) -> Box<dyn AbstractMarket>;
    fn create_account(&self) -> Box<dyn AbstractAccount>;
}