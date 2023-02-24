use crate::patterns::behavioral::strategy::abstract_strategy::AbstractStrategy;

pub struct Average {}

impl Average {
    pub fn new() -> Self {
        Self {}
    }
}

impl AbstractStrategy for Average {

    fn calculate(&self, prices: &Vec<f64>) -> f64 {
        let mut sum = 0.0;
        for p in prices {
            sum += p;
        }
        sum / prices.len() as f64
    }
}