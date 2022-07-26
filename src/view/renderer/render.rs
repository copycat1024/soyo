use crate::{
    tui::{Letter, Quad},
    view::Frame,
};
use std::time::Duration;

pub trait Render: 'static {
    fn layout(&mut self, me: Frame) -> Frame {
        me
    }
    fn render(&self, quad: Quad, letter: &mut Letter);
    fn tick(&mut self, _: Duration) -> bool {
        false
    }
}
