use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};

#[derive(Debug)]
pub struct Config {
    data: String,
}

impl Config {
    pub fn get_instance() -> &'static Mutex<Config> {
        static mut CONF: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            CONF.as_mut_ptr().write(Mutex::new(Config {
                data: "".to_string(),
            }));
        });
    
        unsafe { &*CONF.as_ptr() }
    }

    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }
}