use super::abstract_strategy::AbstractStrategy;

pub struct Prices {
    strategy: Box<dyn AbstractStrategy>,
    prices: Vec<f64>,
}

impl Prices {

    pub fn new(strategy: Box<dyn AbstractStrategy>) -> Self {
        Self {
            strategy,
            prices: vec![0.1, 0.3, 0.4, 0.0, 0.5, 0.2]
        }
    }

    pub fn run(&self) -> f64 {
        self.strategy.calculate(&self.prices)
    }

}