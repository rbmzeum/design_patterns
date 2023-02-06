use super::{
    exchange_name::ExchangeName,
    market_type::MarketType,
    exchange::Exchange,
    trading_operation_error::TradingOperationError,
    trading_operation_status::TradingOperationStatus,
};

// Facade
pub struct TradingAPIv1 {
    exchange: Exchange,
}

impl TradingAPIv1 {

    pub fn new(exchange_name: ExchangeName, market_type: MarketType) -> Self {
        Self {
            exchange: Exchange::new(exchange_name, market_type)
        }
    }

    pub fn buy(&mut self, asset: String, count: f64) -> Result<TradingOperationStatus, TradingOperationError> {
        if count == 0.0 {
            return Err(TradingOperationError{
                code: 201,
                message: "You must be logged in before you can make a request to purchase an asset.".to_string(),
            })
        }
        self.exchange.buy(asset, count)
    }

    pub fn sell(&mut self, asset: String, count: f64) -> Result<TradingOperationStatus, TradingOperationError> {
        if count == 0.0 {
            return Err(TradingOperationError{
                code: 201,
                message: "You must be logged in before you can make a request to purchase an asset.".to_string(),
            })
        }
        self.exchange.sell(asset, count)
    }
}