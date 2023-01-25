pub trait AbstractAccount {
    fn new() -> Self where Self: Sized;
}