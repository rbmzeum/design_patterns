pub mod abstract_strategy;
pub mod strategies;
pub mod prices;

// Example
pub fn behavioral_strategy() {
    use self::{
        strategies::average::Average,
        strategies::median::Median,
        prices::Prices
    };

    let p1 = Prices::new(Box::new(Average::new()));
    let average = p1.run();
    println!("Calculated average price: {}", average);

    let p2 = Prices::new(Box::new(Median::new()));
    let median = p2.run();
    println!("Calculated median price: {}", median);
}