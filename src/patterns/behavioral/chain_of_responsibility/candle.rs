use super::event_handler::EventHandler;
use super::point::Point;

pub struct Candle {
    handler: Option<Box<dyn Fn(&Box<&dyn EventHandler>, &Point)>>,
    parent: Option<Box<dyn EventHandler>>,
}

impl Candle {
    pub fn new() -> Self {
        Self {
            handler: None,
            parent: None,
        }
    }

    pub fn set_parent(&mut self, parent: Box<dyn EventHandler>) {
        self.parent = Some(parent);
    }
}

impl EventHandler for Candle {

    fn handle_click(&self, coords: Point) {
        match &self.handler {
            Some(f) => { f(&Box::new(self), &coords); },
            None => {
                match &self.parent {
                    Some(f) => { f.handle_click(coords); },
                    None => {},
                };
            },
        };
    }

    fn set_handler(&mut self, callback: Box<dyn Fn(&Box<&dyn EventHandler>, &Point)>) {
        self.handler = Some(callback);
    }

}