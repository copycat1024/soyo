use crate::tui::{Letter, Quad};

pub trait Render {
    fn render(&self, quad: Quad, letter: &mut Letter);
}
