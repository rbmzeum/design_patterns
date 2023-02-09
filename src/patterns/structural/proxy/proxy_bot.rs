use super::abstract_bot::AbstractBot;
use super::bot::Bot;

pub struct ProxyBot {
    bot: Bot,
}

impl ProxyBot {
    pub fn new(bot: Bot) -> Self {
        Self {
            bot
        }
    }
}

impl AbstractBot for ProxyBot {

    fn buy(&self, asset: String) {
        print!("Proxy ");
        self.bot.buy(asset);
    }

    fn sell(&self, asset: String) {
        print!("Proxy ");
        self.bot.sell(asset);
    }

}