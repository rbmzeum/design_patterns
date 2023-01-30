use crate::patterns::structural::adapter::abstract_exchange::AbstractExchange;

pub mod abstract_exchange;
pub mod exchanges;

// Example
pub fn structural_adapter() {
    use crate::patterns::structural::adapter::{
        exchanges::binance_exchange::BinanceExchange,
    };
    let exchange = BinanceExchange::new(); // implemented AbstractExchange interface

    let p = exchange.get_pairs_count();
    println!("Pairs count: {}", p);
}