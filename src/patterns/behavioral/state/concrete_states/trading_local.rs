use crate::patterns::behavioral::state::abstract_state::AbstractState;

pub struct TradingLocal {
    exchange_simulator: String,
}

impl TradingLocal {

    pub fn new() -> Self {
        Self {
            exchange_simulator: "Without sending requests".to_string(),
        }
    }

}

impl AbstractState for TradingLocal {

    fn buy(&self) {
        println!("Local: {} -- Buy", self.exchange_simulator);
    }

    fn sell(&self) {
        println!("Local: {} -- Sell", self.exchange_simulator);
    }

}