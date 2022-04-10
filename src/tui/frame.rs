use crate::{
    tui::{Backend, Buffer, Rect},
    util::{Hot, HotRef, Result},
};
use crossterm::{event::Event, style::Color};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

pub struct Frame(Buffer<Slot>);

impl Frame {
    pub fn map_event(&mut self, event: Option<Event>) -> Option<Event> {
        if let Some(event) = event {
            match event {
                Event::Resize(w, h) => {
                    if self.resize(w as i32, h as i32) {
                        Some(event)
                    } else {
                        None
                    }
                }
                _ => Some(event),
            }
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32, letter: Letter) {
        let slot = &mut self.0[(x, y)];

        if slot.z < z {
            *slot.letter = letter;
            slot.z = z;
        }
    }

    pub fn item(&mut self, x: i32, y: i32, z: i32) -> Option<HotRef<Letter>> {
        let slot = &mut self.0[(x, y)];
        if slot.z < z {
            Some(slot.letter.get_ref())
        } else {
            None
        }
    }

    pub fn draw<B: Backend>(&self, backend: &mut B) -> Result {
        writeln!(backend.logger(), "Draw()");

        let mut seq = Sequencer::new(backend);
        let x0 = self.0.rect().x;

        for (c, x, y) in self.0.iter(true) {
            if x == x0 {
                seq.flush()?;
            }
            seq.step(&c.letter, x, y)?;
        }
        seq.flush()?;

        Ok(())
    }

    pub fn clear<B: Backend>(&self, backend: &mut B, c: Color) -> Result {
        backend.bg(c)?.clear()?;
        Ok(())
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self(Buffer::new())
    }
}

impl Frame {
    pub fn resize(&mut self, w: i32, h: i32) -> bool {
        self.0.resize(w, h, Slot::new())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Letter {
    pub fg: Color,
    pub bg: Color,
    pub c: char,
}

#[derive(Clone)]
pub struct Slot {
    z: i32,
    letter: Hot<Letter>,
}

impl Slot {
    fn new() -> Self {
        Self {
            z: 0,
            letter: Letter {
                fg: Color::Reset,
                bg: Color::Reset,
                c: '\0',
            }
            .into(),
        }
    }
}

struct Sequencer<'a, B: Backend> {
    backend: &'a mut B,
    fg: Color,
    bg: Color,
    buf: String,
}

impl<'a, B: Backend> Sequencer<'a, B> {
    fn new(backend: &'a mut B) -> Self {
        Self {
            backend,
            fg: Color::Reset,
            bg: Color::Reset,
            buf: String::from(""),
        }
    }

    fn step(&mut self, letter: &Hot<Letter>, x: i32, y: i32) -> Result {
        if letter.c == '\0' || letter.check() {
            self.flush()?;
        } else {
            self.set_bg(letter.bg)?;
            self.set_fg(letter.fg)?;
            self.place(x, y, letter.c)?;
        }

        Ok(())
    }

    fn flush(&mut self) -> Result {
        if !self.buf.is_empty() {
            self.backend.print(&self.buf)?;
            self.buf.clear();
            self.backend.flush()?;
        }

        Ok(())
    }

    fn place(&mut self, x: i32, y: i32, c: char) -> Result {
        if self.buf.is_empty() {
            self.backend.gotoxy(x, y)?;
        }
        self.buf.push(c);

        Ok(())
    }

    fn set_bg(&mut self, c: Color) -> Result {
        if c != self.bg {
            self.flush()?;
            self.bg = c;
            self.backend.bg(c)?;
        }

        Ok(())
    }

    fn set_fg(&mut self, c: Color) -> Result {
        if c != self.fg {
            self.flush()?;
            self.fg = c;
            self.backend.fg(c)?;
        }

        Ok(())
    }
}
