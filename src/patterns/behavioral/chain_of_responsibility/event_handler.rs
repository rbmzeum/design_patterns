use super::point::Point;

pub trait EventHandler {
    fn handle_click(&self, coords: Point);
    fn set_handler(&mut self, callback: Box<dyn Fn(&Box<&dyn EventHandler>, &Point)>);
}