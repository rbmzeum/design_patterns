pub mod abstract_command;
pub mod buy_command;
pub mod sell_command;
pub mod dispatcher;

// Example
pub fn behavioral_command() {
    use buy_command::BuyCommand;
    use sell_command::SellCommand;
    use dispatcher::Dispatcher;

    let command_buy = BuyCommand{};
    let command_sell = SellCommand{};

    let mut d = Dispatcher::new();
    d.add(Box::new(command_buy));
    d.add(Box::new(command_sell));

    d.dispatch();
}