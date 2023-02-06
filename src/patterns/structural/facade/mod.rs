pub mod trading_api_v1;
pub mod exchange_name;
pub mod market_type;
pub mod exchange;
pub mod market;
pub mod trading_operation_error;
pub mod trading_operation_status;

// Example
pub fn structural_facade() {
    use trading_api_v1::TradingAPIv1;
    use exchange_name::ExchangeName;
    use market_type::MarketType;
    use trading_operation_status::TradingOperationStatus;
    use trading_operation_error::TradingOperationError;

    let mut api = TradingAPIv1::new(ExchangeName::BINANCE, MarketType::MARGIN);
    match api.buy("BTCUSD".to_string(), 3.0) {
        Ok(TradingOperationStatus{asset, count}) => { println!("Buy status: {} - {}", asset, count) },
        Err(TradingOperationError{code, message}) => { println!("Error {}: {}", code, message) },
    };
    match api.buy("ETHUSD".to_string(), 4.0) {
        Ok(TradingOperationStatus{asset, count}) => { println!("Buy status: {} - {}", asset, count) },
        Err(TradingOperationError{code, message}) => { println!("Error {}: {}", code, message) },
    };
    match api.buy("BTCUSD".to_string(), 1.0) {
        Ok(TradingOperationStatus{asset, count}) => { println!("Buy status: {} - {}", asset, count) },
        Err(TradingOperationError{code, message}) => { println!("Error {}: {}", code, message) },
    };
    match api.sell("ETHUSD".to_string(), 8.0) {
        Ok(TradingOperationStatus{asset, count}) => { println!("Buy status: {} - {}", asset, count) },
        Err(TradingOperationError{code, message}) => { println!("Error {}: {}", code, message) },
    };
}