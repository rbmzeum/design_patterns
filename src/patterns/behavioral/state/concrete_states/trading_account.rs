use crate::patterns::behavioral::state::abstract_state::AbstractState;

pub struct TradingAccount {
    connection: String,
}

impl TradingAccount {

    pub fn new() -> Self {
        Self {
            connection: "Real account".to_string(),
        }
    }

}

impl AbstractState for TradingAccount {

    fn buy(&self) {
        println!("Connection: {} -- Buy", self.connection);
    }

    fn sell(&self) {
        println!("Connection: {} -- Sell", self.connection);
    }

}