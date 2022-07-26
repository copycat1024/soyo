use crate::view::{Frame, NodeList};
use std::time::Duration;

pub trait Compose: 'static {
    fn register(&mut self, children: &mut NodeList);
    fn layout(&mut self, me: Frame) -> Frame;
    fn tick(&mut self, _: Duration) -> bool {
        false
    }
}
