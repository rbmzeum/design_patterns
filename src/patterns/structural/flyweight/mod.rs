pub mod signal_type;
pub mod signal;
pub mod signal_factory;
pub mod price;

// Example
pub fn structural_flyweight() {
    use price::Price;
    use signal_factory::SignalFactory;
    use signal_type::SignalType;

    let mut factory = SignalFactory::get_instance().lock().unwrap();

    let prices = vec![5.0, 4.0, 5.0, 6.0, 8.0, 7.0, 3.0, 2.0, 4.0, 6.0, 7.0, 9.0, 10.0, 8.0, 5.0];
    let mut p: Vec<Price> = vec![];
    p.push(Price::new(prices[0], None));
    p.push(Price::new(prices[1], Some(factory.get_signal(SignalType::BUY))));
    p.push(Price::new(prices[2], None));
    p.push(Price::new(prices[3], None));
    p.push(Price::new(prices[4], Some(factory.get_signal(SignalType::SELL))));
    p.push(Price::new(prices[5], None));
    p.push(Price::new(prices[6], None));
    p.push(Price::new(prices[7], None));
    p.push(Price::new(prices[8], Some(factory.get_signal(SignalType::BUY))));
    p.push(Price::new(prices[9], None));
    p.push(Price::new(prices[10], None));
    p.push(Price::new(prices[11], None));
    p.push(Price::new(prices[12], Some(factory.get_signal(SignalType::SELL))));
    p.push(Price::new(prices[13], None));
    p.push(Price::new(prices[14], None));

    println!("Price with sell signal: {:#?}", p[4].output());
}