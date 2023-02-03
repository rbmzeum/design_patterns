use super::error::Error;

pub trait AbstractStock {
    fn set_name(&mut self, name: String);
    fn show(&self);
    fn add(&mut self, stock: Box<dyn AbstractStock>) -> Result<Box<&dyn AbstractStock>, Error>;
    fn remove(&mut self, index: usize) -> Result<Box<&dyn AbstractStock>, Error>;
    fn get_children(&self) -> Result<&Vec<Box<dyn AbstractStock>>, Error>;
}