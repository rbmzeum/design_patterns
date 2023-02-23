use super::{abstract_state::AbstractState, concrete_states::trading_local::TradingLocal};

pub struct RandomTradingStrategy {
    state: Box<dyn AbstractState>,
}

impl RandomTradingStrategy {

    pub fn new() -> Self {
        Self {
            state: Box::new(TradingLocal::new())
        }
    }

    pub(in crate::patterns::behavioral::state) fn set_state(&mut self, state: Box<dyn AbstractState>) {
        self.state = state;
    }

    pub fn buy(&self) {
        self.state.buy();
    }

    pub fn sell(&self) {
        self.state.sell();
    }
}