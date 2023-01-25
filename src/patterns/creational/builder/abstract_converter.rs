pub trait AbstractConverter {
    fn set_name(&mut self, name: String);
    fn set_start(&mut self, timestamp: u64);
    fn set_prices(&mut self, prices: Vec<f64>);
    fn set_volumes(&mut self, volumes: Vec<f64>);
}