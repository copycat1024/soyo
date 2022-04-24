use crate::tui::Letter;
use crossterm::style::Color;

#[derive(Clone)]
pub struct Slot {
    pub z: i32,
    pub letter: Letter,
}

impl Slot {
    pub fn new() -> Self {
        Self {
            z: 0,
            letter: Letter {
                fg: Color::Reset.into(),
                bg: Color::Reset.into(),
                c: '\0'.into(),
            },
        }
    }
}
