use super::{Buffer, Rect};
use crate::util::{Hot, HotRef, Result};
use crossterm::style::Color;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

pub struct Frame(Buffer<Slot>);

impl Frame {
    pub fn new() -> Self {
        Self(Buffer::new())
    }

    pub fn resize(&mut self, w: i32, h: i32) -> bool {
        self.0.resize(w, h, Slot::new())
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

    pub fn draw(&self) -> Result {
        let mut seq = Sequencer::new(self);
        let x0 = self.0.rect().x;

        for (c, x, y) in self.0.iter(true) {
            seq.step(&c.letter, x, y, x == x0)?;
        }
        seq.flush(true)?;

        Ok(())
    }

    pub fn clear(&self, c: Color) -> Result {
        Sequencer::new(self).bg(c)?.clear().map(|_| ())
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

struct Sequencer<'a> {
    src: &'a Frame,
    fg: Color,
    bg: Color,
    buf: String,
    place: bool,
}

impl<'a> Sequencer<'a> {
    fn new(src: &'a Frame) -> Self {
        Self {
            src,
            fg: Color::Reset,
            bg: Color::Reset,
            buf: String::from(""),
            place: false,
        }
    }

    fn step(&mut self, letter: &Hot<Letter>, x: i32, y: i32, sol: bool) -> Result {
        if sol {
            self.flush(false)?;
        }

        if letter.c == '\0' && letter.check() {
            self.flush(false)?;
        } else {
            if letter.bg != self.bg {
                self.set_bg(letter.bg)?;
            }

            if letter.fg != self.fg {
                self.set_fg(letter.fg)?;
            }

            self.place(x, y, letter.c)?;
        }

        Ok(())
    }

    fn flush(&mut self, p: bool) -> Result {
        if !self.buf.is_empty() {
            self.place = p;
            self.print(&self.buf)?;
            self.buf.clear();
        }

        Ok(())
    }

    fn place(&mut self, x: i32, y: i32, c: char) -> Result {
        if !self.place {
            self.gotoxy(x, y)?;
            self.place = true;
        }
        self.buf.push(c);

        Ok(())
    }

    fn set_bg(&mut self, c: Color) -> Result {
        self.flush(true)?;
        self.bg = c;
        self.bg(c)?;

        Ok(())
    }

    fn set_fg(&mut self, c: Color) -> Result {
        self.flush(true)?;
        self.fg = c;
        self.fg(c)?;

        Ok(())
    }
}

impl<'a> Sequencer<'a> {
    fn gotoxy(&self, x: i32, y: i32) -> Result<&Self> {
        use crossterm::{cursor::MoveTo, QueueableCommand};
        use std::io::stdout;

        stdout().queue(MoveTo(x as u16, y as u16))?;

        Ok(self)
    }

    fn fg(&self, c: Color) -> Result<&Self> {
        use crossterm::{style::SetForegroundColor, QueueableCommand};
        use std::io::stdout;

        stdout().queue(SetForegroundColor(c))?;

        Ok(self)
    }

    fn bg(&self, c: Color) -> Result<&Self> {
        use crossterm::{style::SetBackgroundColor, QueueableCommand};
        use std::io::stdout;

        stdout().queue(SetBackgroundColor(c))?;

        Ok(self)
    }

    fn print<T: Display>(&self, txt: &T) -> Result<&Self> {
        use crossterm::{style::Print, QueueableCommand};
        use std::io::stdout;

        stdout().queue(Print(txt))?;

        Ok(self)
    }

    fn clear(&self) -> Result<&Self> {
        use crossterm::{
            style::ResetColor,
            terminal::{Clear, ClearType},
            QueueableCommand,
        };
        use std::io::stdout;

        stdout().queue(ResetColor)?.queue(Clear(ClearType::All))?;

        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Rect};

    #[test]
    fn test_item() {
        let mut f = Frame::new();
        f.resize(4, 4);
        if let Some(mut l) = f.item(1, 1, 1) {
            l.c = 'A';
        }

        for (c, x, y) in f.0.iter(true) {
            if x == 1 && y == 1 {
                assert_eq!(f.0[(x, y)].letter.c, 'A', "at ({},{})", x, y);
            } else {
                assert_eq!(f.0[(x, y)].letter.c, '\0', "at ({},{})", x, y);
            }
        }
    }
}
