pub trait AbstractMarket {
    fn new() -> Self where Self: Sized;
    fn get_pairs_count(&self) -> usize;
}