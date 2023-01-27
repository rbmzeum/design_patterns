use crate::patterns::creational::builder::abstract_converter::AbstractConverter;

pub struct BinaryConverter {
    name: String,
    start_timestamp: u64,
    prices: Vec<f64>,
    volumes: Vec<f64>,
}

impl BinaryConverter {
    
    pub fn default() -> Self {
        Self {
            name: "".to_string(),
            start_timestamp: 0,
            prices: vec![],
            volumes: vec![],
        }
    }

    pub fn get_file_name(&self) -> String {
        format!("{}.dat", self.name)
    }

    pub fn get_offset_bytes(&self) -> u64 {
        let january_27_2023 = 1674817945;
        self.start_timestamp - january_27_2023
    }

    pub fn get_binary_data(&self) -> Vec<u8> {
        if self.prices.len() != self.volumes.len() {
            // painc!("Prices length is not equal with volumes length");
        }

        let mut buffer = vec![];
        for (index, &price) in self.prices.iter().enumerate() {
            let p = price.to_be_bytes();
            for i in 0..7 {
                buffer.push(p[i]);
            }

            let v = self.volumes[index].to_be_bytes();
            for i in 0..7 {
                buffer.push(v[i]);
            }
        }
        
        buffer
    }

}

impl AbstractConverter for BinaryConverter {

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_start(&mut self, timestamp: u64) {
        self.start_timestamp = timestamp;
    }

    fn set_prices(&mut self, prices: Vec<f64>) {
        self.prices = prices;
    }

    fn set_volumes(&mut self, volumes: Vec<f64>) {
        self.volumes = volumes;
    }

}