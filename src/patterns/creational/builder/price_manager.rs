use crate::patterns::creational::builder::{
    abstract_converter::AbstractConverter,
};

use std::time::SystemTime;

pub struct PriceManager {
    pub prices: Vec<f64>,
    pub volumes: Vec<f64>,
}

impl PriceManager {

    pub fn construct(&mut self, converter: &mut dyn AbstractConverter) {
        converter.set_name("Rubble".to_string());

        // TODO: инкапсулировать логику в конкретных строителей, а передавать опциональный параметр, либо CURRENT_TIME, либо конкретное значение
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {
                converter.set_start(n.as_secs());
            },
            Err(_) => {
                panic!("SystemTime before UNIX EPOCH!");
            },
        };

        // FIXME: обернуть в Box или Rc / Cell (разобраться как правильней), чтобы не использовать clone()
        converter.set_prices(self.prices.clone());
        converter.set_volumes(self.volumes.clone());
    }
}