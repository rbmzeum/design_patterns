use super::{
    market::Market,
    exchange_name::ExchangeName,
    market_type::MarketType,
    trading_operation_error::TradingOperationError,
    trading_operation_status::TradingOperationStatus,
};

pub struct Exchange {
    _name: ExchangeName,
    market: Market,
}

impl Exchange {
    pub fn new(exchange_name: ExchangeName, market_type: MarketType) -> Self {
        Self {
            _name: exchange_name,
            market: Market::new(market_type),
        }
    }

    pub fn buy(&mut self, asset: String, count: f64) -> Result<TradingOperationStatus, TradingOperationError> {
        let c = self.market.get_count(&asset) + count;
        self.market.set_count(&asset, c)
    }

    pub fn sell(&mut self, asset: String, count: f64) -> Result<TradingOperationStatus, TradingOperationError> {
        let c = self.market.get_count(&asset) - count;
        self.market.set_count(&asset, c)
    }
}