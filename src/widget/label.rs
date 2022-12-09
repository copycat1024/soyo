use crate::{
    tui::{Letter, Quad},
    util::{FlexVec, HAlign},
    view::Render,
};
use std::fmt::Arguments;

pub struct Label {
    text: FlexVec<char>,
    ha: HAlign,
}

impl Label {
    fn align(&self, pos: Quad) -> i32 {
        let w1 = self.text.len();
        let w2 = pos.w;
        match self.ha {
            HAlign::Center => (w2 - w1) / 2,
            HAlign::Left => 0,
            HAlign::Right => w2 - w1,
        }
    }

    pub fn write_fmt(&mut self, fmt: Arguments<'_>) {
        write!(self.text, "{}", fmt);
    }

    pub fn set_align(&mut self, ha: HAlign) {
        self.ha = ha
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
            ha: HAlign::Center,
        }
    }
}
