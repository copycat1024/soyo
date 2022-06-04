use super::{Render, Text};
use crate::tui::{Color, Letter, Quad};

#[derive(Default)]
pub struct Label {
    pub text: Text,
}

impl Label {
    fn align(&self, pos: Quad) -> i32 {
        let w1 = self.text.data.len() as i32;
        let w2 = pos.w;
        (w2 - w1) / 2
    }
}

impl Render for Label {
    fn render(&self, quad: Quad, letter: &mut Letter) {
        *letter.c = self.text[quad.x - self.align(quad)];
        *letter.bg = Color::RED;
    }
}
