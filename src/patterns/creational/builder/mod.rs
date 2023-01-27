pub mod converters;

pub mod abstract_converter;
pub mod price_manager;

// Example
pub fn creational_builder() {
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
    println!("CSV FILE NAME: {}", file_name);
    println!("CSV DATA: {:#?}", csv_data);

    let mut binary_converter = BinaryConverter::default();
    pm.construct(&mut binary_converter);
    let binary_data = binary_converter.get_binary_data();
    println!("BINARY DATA: {:#?}", binary_data);
}