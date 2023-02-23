pub mod abstract_state;
pub mod random_trading_strategy;
pub mod concrete_states;

// Example
pub fn behavioral_state() {
    use random_trading_strategy::RandomTradingStrategy;
    use concrete_states::{
        trading_account::TradingAccount,
        trading_demo_account::TradingDemoAccount,
    };

    let mut rts = RandomTradingStrategy::new();

    rts.buy();
    rts.sell();

    rts.set_state(Box::new(TradingDemoAccount::new()));

    rts.buy();
    rts.sell();

    rts.set_state(Box::new(TradingAccount::new()));

    rts.buy();
    rts.sell();
}