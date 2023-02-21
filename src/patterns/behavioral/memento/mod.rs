pub mod memento_portfolio;
pub mod target_portfolio;

// Example
pub fn behavioral_memento() {
    use self::target_portfolio::TargetPortfolio;

    let mut tp = TargetPortfolio::new();

    tp.set_usd(0.1);
    tp.set_rub(0.2);
    tp.set_btc(0.3);
    tp.set_eth(0.4);
    let mem = tp.create_memento();

    tp.set_usd(0.25);
    tp.set_rub(0.25);
    tp.set_btc(0.25);
    tp.set_eth(0.25);

    println!("Target portfolio shares: [usd: {}, rub: {}, btc: {}, eth: {}]", tp.get_usd(), tp.get_rub(), tp.get_btc(), tp.get_eth());
    tp.set_memento(mem);
    println!("Restored target portfolid shares: [usd: {}, rub: {}, btc: {}, eth: {}]", tp.get_usd(), tp.get_rub(), tp.get_btc(), tp.get_eth());
}