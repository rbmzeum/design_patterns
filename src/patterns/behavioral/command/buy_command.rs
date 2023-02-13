use super::abstract_command::AbstractCommand;

pub struct BuyCommand {}

impl AbstractCommand for BuyCommand {
    fn execute(&self) {
        println!("Buy");
    }
}