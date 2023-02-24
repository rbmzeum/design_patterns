pub trait AbstractStrategy {
    fn calculate(&self, prices: &Vec<f64>) -> f64;
}