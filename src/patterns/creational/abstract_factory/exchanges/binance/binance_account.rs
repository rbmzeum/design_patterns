use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use home::home_dir;

use binance::account::Account;
use binance::api::Binance;

use crate::patterns::creational::abstract_factory::exchanges::abstract_account::AbstractAccount;
use confy::{load, ConfyError};

pub struct BinanceAccount {
    account: Account,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub usr_home: Option<PathBuf>,
    pub public_key: String,
    pub secter_key: String,
    pub blocks_count: u64,
}

impl ::std::default::Default for Settings {

    fn default() -> Self {
        Self {
            usr_home: home_dir(),
            public_key: "".to_string(),
            secter_key: "".to_string(),
            blocks_count: 512, // позволяет сохранить 512*512 ежеминутных значений цен типа f64 в одном файле, что составляет порядка 3 мегабайт данных за год по одной криптопаре
        }
    }
    
}

impl AbstractAccount for BinanceAccount {

    fn new() -> Self {
        let cfg: Result<Settings, ConfyError> = load("autobot"); // load from "/home/<user>/.config/autobot/autobot.toml"
        let settings = match cfg {
            Ok(settings) => settings,
            Err(_e) => Settings::default(),
        };

        let api_key = Some(settings.public_key.clone());
        let secret_key = Some(settings.secter_key.clone());

        Self {
            account: Binance::new(api_key, secret_key)
        }
    }

    fn can_trade(&self) -> bool {
        match self.account.get_account() {
            Ok(account_information) => {
                account_information.can_trade
            },
            Err(e) => {
                panic!("Error: {:?}", e)
            },
        }
    }

}