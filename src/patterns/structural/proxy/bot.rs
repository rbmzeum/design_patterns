use super::abstract_bot::AbstractBot;

pub struct Bot {}

impl AbstractBot for Bot {

    fn buy(&self, asset: String) {
        println!("Buy {}", asset);
    }

    fn sell(&self, asset: String) {
        println!("Sell {}", asset);
    }

}