use super::signal_type::SignalType;

#[derive(Debug)]
pub struct Signal {
    signal_type: SignalType,
    message: String,
}

impl Signal {
    pub fn new(signal_type: SignalType, message: &str) -> Self {
        Self {
            signal_type,
            message: message.to_string(),
        }
    }

    pub fn output(&self) -> String {
        let t = match self.signal_type {
            SignalType::BUY => "BUY",
            SignalType::SELL => "SELL",
        };
        format!("{}, {}", t, self.message)
    }
}