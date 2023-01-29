pub mod config;

// Example
pub fn creational_singleton() {
    use crate::patterns::creational::singleton::config::Config;

    let config = Config::get_instance();
    config.lock().unwrap().set_data("This is singleton".to_string());
    println!("One: {}", config.lock().unwrap().get_data());

    let the_same_config = Config::get_instance();
    println!("Two: {}", the_same_config.lock().unwrap().get_data());
}