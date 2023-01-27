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
    println!("CSV file name: {}", file_name);
    println!("CSV data: {:#?}", csv_data);

    let mut binary_converter = BinaryConverter::default();
    pm.construct(&mut binary_converter);
    let file_name = binary_converter.get_file_name();
    let offset_bytes = binary_converter.get_offset_bytes();
    let binary_data = binary_converter.get_binary_data();
    println!("Binary file name: {}", file_name);
    println!("Binary offset bytes: {}", offset_bytes);
    println!("Binary data: {:#?}", binary_data);
}