pub mod abstract_portfolio;
pub mod portfolios;
pub mod portfolio_manager;

// Example
pub fn creational_prototype() {
    use crate::patterns::creational::prototype::portfolio_manager::PortfolioManager;

    let mut pm = PortfolioManager::new();
    pm.create_objects();
    pm.output();
}