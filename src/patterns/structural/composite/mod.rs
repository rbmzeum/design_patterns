pub mod error;
pub mod abstract_stock;
pub mod asset;
pub mod portfolio;

// Example
pub fn structural_composite() {
    use abstract_stock::AbstractStock;
    use asset::Asset;
    use portfolio::Portfolio;

    let mut rub = Asset::new();
    rub.set_name("Ruble".to_string());
    rub.show();

    let mut usd = Asset::new();
    usd.set_name("Dollar".to_string());
    let r = usd.add(Box::new(rub.clone()));
    match r {
        Ok(_result) => {
            panic!("Should not be here!");
        },
        Err(error) => {
            println!("Error {}: {}", error.get_code(), error.get_message());
        },
    }
    usd.show();

    let mut btc = Asset::new();
    btc.set_name("Bitcoin".to_string());
    btc.show();

    let mut eth = Asset::new();
    eth.set_name("Etherium".to_string());
    eth.show();

    let mut xmr = Asset::new();
    xmr.set_name("Monero".to_string());
    xmr.show();

    let mut p1 = Portfolio::new();
    p1.set_name("Rus".to_string());
    let _r11 = p1.add(Box::new(rub));
    let _r12 = p1.add(Box::new(eth));
    p1.show();

    let mut p2 = Portfolio::new();
    p2.set_name("Eng".to_string());
    let _r21 = p2.add(Box::new(usd));
    let _r22 = p2.add(Box::new(btc));
    p2.show();

    let mut mix = Portfolio::new();
    mix.set_name("Mixed".to_string());
    let _r31 = mix.add(Box::new(p1));
    let _r32 = mix.add(Box::new(xmr));
    let _r33 = mix.add(Box::new(p2));
    mix.show();
}