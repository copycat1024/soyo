use crate::view::{Frame, NodeList};

pub trait Compose: 'static {
    fn register(&mut self, children: &mut NodeList);
    fn layout(&mut self, me: Frame) -> Frame;
    fn tick(&mut self, _: u64) -> bool {
        false
    }
}
