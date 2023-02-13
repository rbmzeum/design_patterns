use super::abstract_command::AbstractCommand;

pub struct Dispatcher {
    qeue: Vec<Box<dyn AbstractCommand>>,
}

impl Dispatcher {

    pub fn new() -> Self {
        Self {
            qeue: vec![],
        }
    }

    pub fn add(&mut self, command: Box<dyn AbstractCommand>) {
        self.qeue.push(command);
    }

    pub fn dispatch(&self) {
        for c in &self.qeue {
            c.execute();
        }
    }

}