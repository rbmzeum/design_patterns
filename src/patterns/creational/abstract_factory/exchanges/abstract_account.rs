pub trait AbstractAccount {
    fn new() -> Self where Self: Sized;
    fn can_trade(&self) -> bool;
}