pub mod abstract_observer;
pub mod abstract_subject;
pub mod shares;
pub mod bot;
pub mod portfolio;

pub fn behavioral_observer() {
    use self::bot::Bot;
    use crate::patterns::behavioral::observer::{abstract_subject::AbstractSubject, portfolio::Portfolio};

    let mut bot = Bot::new();
    let p1 = Portfolio::new();
    let p2 = Portfolio::new();

    bot.attach(&p1);
    bot.attach(&p2);

    bot.set_usd(0.3);
    bot.set_btc(0.4);

    p1.show();
    p2.show();

    bot.detach(0);

    bot.set_usd(0.25);
    bot.set_btc(0.25);

    p1.show();
    p2.show();
}