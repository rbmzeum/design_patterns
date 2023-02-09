pub mod abstract_bot;
pub mod bot;
pub mod proxy_bot;

// Example
pub fn structural_proxy() {
    use abstract_bot::AbstractBot;
    use bot::Bot;
    use proxy_bot::ProxyBot;

    let bot = Bot{};
    bot.buy("BTCUSD".to_string());
    bot.sell("BTCUSD".to_string());

    let proxy_bot = ProxyBot::new(bot);
    proxy_bot.buy("BTCUSD".to_string());
    proxy_bot.sell("BTCUSD".to_string());
}