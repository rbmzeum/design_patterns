pub mod exchanges;

// Example
pub fn crational_abstract_factory() {
    use crate::patterns::creational::abstract_factory::exchanges::{
        abstract_exchange_factory::AbstractExchangeFactory,
        binance::binance_exchange_factory::BinanceExchangeFactory,
    };

    let factory = BinanceExchangeFactory{};

    let market = factory.create_market();
    let pairs_count = market.get_pairs_count();
    println!("Pairs count: {:#?}", pairs_count);

    let account = factory.create_account();
    let can_trade = account.can_trade();
    println!("Account can trade: {:#?}", can_trade);
}