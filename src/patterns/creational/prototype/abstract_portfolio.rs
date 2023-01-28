pub trait AbstractPortfolio {
    fn set_id(&mut self, id: u64);
    fn get_name(&self) -> String;
}