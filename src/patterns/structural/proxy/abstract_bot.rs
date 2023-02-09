pub trait AbstractBot {
    fn buy(&self, asset: String);
    fn sell(&self, asset: String);
}