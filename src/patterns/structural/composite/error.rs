pub struct Error {
    code: u64,
    message: String,
}

impl Error {
    pub fn new(code: u64, message: String) -> Self {
        Self {
            code: code,
            message: message
        }
    }

    pub fn get_code(&self) -> u64 {
        self.code
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }
}