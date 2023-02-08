#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum SignalType {
    BUY,
    SELL,
    // SHORT_SELLING,
    // CLOSE_SHORT,
    // SELL_AND_SHORT_SELLING,
    // CLOSE_SHORT_AND_BUY,
}