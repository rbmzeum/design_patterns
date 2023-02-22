use super::abstract_observer::AbstractObserver;

pub trait AbstractSubject<'a, T: AbstractObserver> {
    fn attach(&mut self, observer: &'a T);
    fn detach(&mut self, index: usize);
    fn notify(&self);
}