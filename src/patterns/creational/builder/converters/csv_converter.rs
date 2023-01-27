use crate::patterns::creational::builder::abstract_converter::AbstractConverter;

pub struct CsvConverter {
    name: String,
    // start_timestamp: u64,
    prices: Vec<f64>,
    volumes: Vec<f64>,
}

impl CsvConverter {
    
    pub fn default() -> Self {
        Self {
            name: "".to_string(),
            // start_timestamp: 0,
            prices: vec![],
            volumes: vec![],
        }
    }

    pub fn get_file_name(&self) -> String {
        format!("{}.csv", self.name)
    }

    pub fn get_csv_data(&self) -> Vec<String> {
        if self.prices.len() != self.volumes.len() {
            // painc!("Prices length is not equal with volumes length");
        }

        let mut rows = vec![];
        for (index, &price) in self.prices.iter().enumerate() {
            let row = format!("{}; {}", price, self.volumes[index]);
            rows.push(row);
        }
        
        rows
    }

}

impl AbstractConverter for CsvConverter {

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_start(&mut self, _timestamp: u64) {
        // self.start_timestamp = timestamp;
    }

    fn set_prices(&mut self, prices: Vec<f64>) {
        self.prices = prices;
    }

    fn set_volumes(&mut self, volumes: Vec<f64>) {
        self.volumes = volumes;
    }

}