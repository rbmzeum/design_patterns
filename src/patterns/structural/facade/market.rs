use std::collections::HashMap;
use super::{
    market_type::MarketType,
    trading_operation_error::TradingOperationError,
    trading_operation_status::TradingOperationStatus,
};

pub struct Market {
    market_type: MarketType,
    counts: HashMap<String, f64>,
}

impl Market {
    pub fn new(market_type: MarketType) -> Self {
        Self {
            market_type: market_type,
            counts: HashMap::new(),
        }
    }

    pub fn set_count(&mut self, asset: &String, count: f64) -> Result<TradingOperationStatus, TradingOperationError> {
        if count < 0.0 && self.market_type == MarketType::SPOT {
            return Err(TradingOperationError {
                code: 401,
                message: "Quantity cannot be less than zero on the spot market".to_string(),
            })
        }
        if self.counts.contains_key(asset) {
            self.counts.remove(asset);
        }
        self.counts.insert(asset.clone(), count);

        Ok(TradingOperationStatus{ asset: asset.clone(), count: count })
    }

    pub fn get_count(&mut self, asset: &String) -> f64 {
        match self.counts.get(asset) {
            Some(v) => v.clone(),
            None => 0.0,
        }
    }
}