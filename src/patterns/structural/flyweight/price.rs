use super::signal::Signal;
use std::rc::Rc;

#[derive(Debug)]
pub struct Price {
    operation: Option<Rc<Signal>>,
    value: f64,
}

impl Price {
    pub fn new(value: f64, operation: Option<Rc<Signal>>) -> Self {
        Self {
            operation,
            value
        }
    }

    pub fn output(&self) -> String {
        format!("{}, {}", self.value, match &self.operation {
            Some(v) => v.output(),
            None => "".to_string(),
        })
    }
}