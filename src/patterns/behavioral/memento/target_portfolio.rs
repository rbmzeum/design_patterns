use super::memento_portfolio::MementoPortfolio;

pub struct TargetPortfolio {
    usd: f64,
    rub: f64,
    btc: f64,
    eth: f64,
}

impl TargetPortfolio {

    pub fn new() -> Self {
        Self {
            usd: 0.0,
            rub: 0.0,
            btc: 0.0,
            eth: 0.0,
        }
    }

    pub fn create_memento(&self) -> MementoPortfolio {
        let mut memento = MementoPortfolio::new();
        memento.set_shares(self.usd, self.rub, self.btc, self.eth);
        memento
    }

    pub fn set_memento(&mut self, memento: MementoPortfolio) {
        let shares = memento.get_shares();
        (self.usd, self.rub, self.btc, self.eth) = (shares[0], shares[1], shares[2], shares[3]);
    }

    pub fn set_usd(&mut self, share: f64) {
        self.usd = share;
    }

    pub fn set_rub(&mut self, share: f64) {
        self.rub = share;
    }

    pub fn set_btc(&mut self, share: f64) {
        self.btc = share;
    }

    pub fn set_eth(&mut self, share: f64) {
        self.eth = share;
    }

    pub fn get_usd(&self) -> f64 {
        self.usd
    }

    pub fn get_rub(&self) -> f64 {
        self.rub
    }

    pub fn get_btc(&self) -> f64 {
        self.btc
    }

    pub fn get_eth(&self) -> f64 {
        self.eth
    }

}