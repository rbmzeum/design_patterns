mod patterns;

fn main() {
    println!("Hello, world!");

    // 1. Creational / Abstract Factory
    use crate::patterns::creational::abstract_factory::exchanges::abstract_exchange_factory::AbstractExchangeFactory;
    use crate::patterns::creational::abstract_factory::exchanges::binance::binance_exchange_factory::BinanceExchangeFactory;

    let factory = BinanceExchangeFactory{};

    let market = factory.create_market();
    let account = factory.create_account();

    // 2. Creational / Builder
    use crate::patterns::creational::builder::{
        price_manager::PriceManager,
        converters::csv_converter::CsvConverter,
        converters::binary_converter::BinaryConverter,
    };

    let mut pm = PriceManager{
        prices: vec![3.0, 4.0],
        volumes: vec![300.0, 400.0],
    };

    let mut csv_converter = CsvConverter::default();
    pm.construct(&mut csv_converter);
    let file_name = csv_converter.get_file_name();
    let csv_data = csv_converter.get_csv_data();

    let mut binary_converter = BinaryConverter::default();
    pm.construct(&mut binary_converter);
    let binary_data = binary_converter.get_binary_data();

}
