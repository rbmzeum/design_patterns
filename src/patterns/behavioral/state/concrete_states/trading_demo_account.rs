use crate::patterns::behavioral::state::abstract_state::AbstractState;

pub struct TradingDemoAccount {
    connection: String,
}

impl TradingDemoAccount {

    pub fn new() -> Self {
        Self {
            connection: "Demo account".to_string(),
        }
    }

}

impl AbstractState for TradingDemoAccount {

    fn buy(&self) {
        println!("Connection: {} -- Buy", self.connection);
    }

    fn sell(&self) {
        println!("Connection: {} -- Sell", self.connection);
    }

}