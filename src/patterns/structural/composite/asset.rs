use super::abstract_stock::AbstractStock;
use super::error::Error;

#[derive(Clone)]
pub struct Asset {
    name: String,
}

impl Asset {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}

impl AbstractStock for Asset {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn show(&self) {
        println!("Name: {}", self.name);
    }

    fn add(&mut self, _stock: Box<dyn AbstractStock>) -> Result<Box<&dyn AbstractStock>, Error> {
        Err(Error::new(
            401,
            "You can not add elements to the leaf element".to_string()
        ))
    }

    fn remove(&mut self, _index: usize) -> Result<Box<&dyn AbstractStock>, Error> {
        Err(Error::new(
            402,
            "You can not remove elements from the leaf element".to_string()
        ))
    }

    fn get_children(&self) -> Result<&Vec<Box<dyn AbstractStock>>, Error> {
        Err(Error::new(
            403,
            "Leaf element cannot contain subelements".to_string()
        ))
    }
}