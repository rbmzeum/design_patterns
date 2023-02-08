use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};
use std::rc::Rc;

use super::signal_type::SignalType;
use super::signal::Signal;

#[derive(Debug)]
pub struct SignalFactory {
    signals_map: HashMap<SignalType, Rc<Signal>>,
}

impl SignalFactory {
    pub fn get_instance() -> &'static Mutex<SignalFactory> {
        static mut SIGNALS_MAP: MaybeUninit<Mutex<SignalFactory>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            SIGNALS_MAP.as_mut_ptr().write(Mutex::new(SignalFactory {
                signals_map: HashMap::new(),
            }));
        });
    
        unsafe { &*SIGNALS_MAP.as_ptr() }
    }

    pub fn get_signal(&mut self, signal_type: SignalType) -> Rc<Signal> {
        if !self.signals_map.contains_key(&signal_type) {
            let signal = Signal::new(
                signal_type.clone(),
                match signal_type {
                    SignalType::BUY => "Buy",
                    SignalType::SELL => "Sell",
                    // SignalType::SHORT_SELLING => "Short selling",
                    // SignalType::CLOSE_SHORT => "Close short",
                    // SignalType::SELL_AND_SHORT_SELLING => "Sell & short selling",
                    // SignalType::CLOSE_SHORT_AND_BUY => "Close short & buy",
                }
            );
            self.signals_map.insert(signal_type.clone(), Rc::new(signal));
        }
        self.signals_map.get(&signal_type).unwrap().clone()
    }
}