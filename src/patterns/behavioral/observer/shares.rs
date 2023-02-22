pub struct Shares {
    usd: f64,
    btc: f64,
}

impl Shares {

    pub fn new() -> Self {
        Self {
            usd: 0.0,
            btc: 0.0,
        }
    }

    pub fn set_usd(&mut self, count: f64) {
        self.usd = count;
    }

    pub fn set_btc(&mut self, count: f64) {
        self.btc = count;
    }

    pub fn get_usd(&self) -> f64 {
        self.usd
    }

    pub fn get_btc(&self) -> f64 {
        self.btc
    }

}