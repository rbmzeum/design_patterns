use crate::patterns::behavioral::iterator::{
    abstract_item::AbstractItem,
    item_type::ItemType,
};

pub struct Strategy {
    name: String,
}

impl Strategy {

    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

}

impl AbstractItem for Strategy {

    fn get_type(&self) -> ItemType {
        ItemType::STRATEGY
    }

    fn rename(&mut self, name: String) {
        self.name = name;
    }

    fn print_name(&self) {
        println!("Strategy: {}", self.name);
    }

}