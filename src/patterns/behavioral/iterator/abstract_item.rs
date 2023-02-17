use super::item_type::ItemType;

pub trait AbstractItem {

    fn get_type(&self) -> ItemType;
    fn rename(&mut self, name: String);
    fn print_name(&self);

}