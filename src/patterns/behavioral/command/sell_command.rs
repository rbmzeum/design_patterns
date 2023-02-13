use super::abstract_command::AbstractCommand;

pub struct SellCommand {}

impl AbstractCommand for SellCommand {
    fn execute(&self) {
        println!("Sell");
    }
}