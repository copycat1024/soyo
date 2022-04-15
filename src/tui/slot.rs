use super::{Backend, Buffer, Rect};
use crate::util::{Hot, Result};
use crossterm::{event::Event, style::Color};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

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
