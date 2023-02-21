pub struct MementoPortfolio {
    shares: [f64; 4],
}

impl MementoPortfolio {

    pub(in crate::patterns::behavioral::memento) fn new() -> Self {
        Self {
            shares: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub(in crate::patterns::behavioral::memento) fn set_shares(&mut self, usd: f64, rub: f64, btc: f64, eth: f64) {
        self.shares = [usd, rub, btc, eth];
    }

    pub(in crate::patterns::behavioral::memento) fn get_shares(&self) -> [f64; 4] {
        self.shares
    }

}