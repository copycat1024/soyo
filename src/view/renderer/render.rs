use crate::{
    tui::{Letter, Quad},
    view::Frame,
};

pub trait Render: 'static {
    fn render(&self, quad: Quad, letter: &mut Letter);

    fn layout(&mut self, _: &mut Frame) {}

    fn tick(&mut self, _: u64) -> bool {
        false
    }
}
