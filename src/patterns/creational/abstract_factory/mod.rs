pub mod exchanges;

// Example
pub fn crational_abstract_factory() {
    use crate::patterns::creational::abstract_factory::exchanges::{
        abstract_exchange_factory::AbstractExchangeFactory,
        binance::binance_exchange_factory::BinanceExchangeFactory,
    };

    let factory = BinanceExchangeFactory{};

    let market = factory.create_market();
    let account = factory.create_account();

    // TODO: добавить пример вызова методов из market и account
}