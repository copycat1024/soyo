use super::{FlexVec, Render};
use crate::tui::{Color, Letter, Quad};

pub struct Label {
    pub text: FlexVec<char>,
}

impl Label {
    fn align(&self, pos: Quad) -> i32 {
        let w1 = self.text.len();
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
