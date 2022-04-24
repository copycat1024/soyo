use crate::tui::{Color, Letter};

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
                fg: Color::WHITE.into(),
                bg: Color::BLACK.into(),
                c: '\0'.into(),
            },
        }
    }
}
