use crate::{
    tui::{Letter, Quad},
    view::Frame,
};

pub trait Render: 'static {
    fn layout(&mut self, me: Frame) -> Frame {
        me
    }
    fn render(&self, quad: Quad, letter: &mut Letter);
}
