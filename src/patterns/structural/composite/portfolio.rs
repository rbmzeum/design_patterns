use super::abstract_stock::AbstractStock;
use super::error::Error;

pub struct Portfolio {
    name: String,
    children: Vec<Box<dyn AbstractStock>>,
}

impl Portfolio {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            children: vec![],
        }
    }
}

impl AbstractStock for Portfolio {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn show(&self) {
        println!("Name: {}", self.name);
        for (index, child) in self.children.iter().enumerate() {
            print!("Cildren {}: ", index);
            child.show();
        }
    }

    fn add(&mut self, stock: Box<dyn AbstractStock>) -> Result<Box<&dyn AbstractStock>, Error> {
        self.children.push(stock);
        Ok(Box::new(self))
    }

    fn remove(&mut self, index: usize) -> Result<Box<&dyn AbstractStock>, Error> {
        self.children.remove(index);
        Ok(Box::new(self))
    }

    fn get_children(&self) -> Result<&Vec<Box<dyn AbstractStock>>, Error> {
        Ok(&self.children)
    }
}