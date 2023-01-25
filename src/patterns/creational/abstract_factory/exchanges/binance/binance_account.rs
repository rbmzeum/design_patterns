use binance::account::Account;
use binance::api::Binance;

use crate::patterns::creational::abstract_factory::exchanges::abstract_account::AbstractAccount;

pub struct BinanceAccount {
    account: Account,
}

impl AbstractAccount for BinanceAccount {

    fn new() -> Self {
        Self {
            account: Binance::new(None, None)
        }
    }

}