use crate::{
    tui::{Letter, Quad},
    util::FlexVec,
    view::Render,
};
use std::fmt::Arguments;

pub struct Label {
    text: FlexVec<char>,
}

impl Label {
    fn align(&self, pos: Quad) -> i32 {
        let w1 = self.text.len();
        let w2 = pos.w;
        (w2 - w1) / 2
    }

    pub fn write_fmt(&mut self, fmt: Arguments<'_>) {
        write!(self.text, "{}", fmt);
    }
}

impl Render for Label {
    fn render(&self, quad: Quad, letter: &mut Letter) {
        *letter.c = self.text[quad.x - self.align(quad)];
    }
}

impl Default for Label {
    fn default() -> Self {
        Self {
            text: FlexVec::new(' '),
        }
    }
}
