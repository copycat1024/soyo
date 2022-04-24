use crate::{tui::Color, util::Hot};

#[derive(Clone)]
pub struct Letter {
    pub fg: Hot<Color>,
    pub bg: Hot<Color>,
    pub c: Hot<char>,
}

impl Letter {
    pub fn hot(&self) -> bool {
        self.fg.hot() || self.bg.hot() || (self.c.hot() && *self.c != '\0')
    }
}
