use crate::patterns::behavioral::iterator::{
    abstract_item::AbstractItem,
    item_type::ItemType,
};

pub struct Indicator {
    name: String,
}

impl Indicator {

    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

}

impl AbstractItem for Indicator {

    fn get_type(&self) -> ItemType {
        ItemType::INDICATOR
    }

    fn rename(&mut self, name: String) {
        self.name = name;
    }

    fn print_name(&self) {
        println!("Indicator: {}", self.name);
    }

}