use crate::{
    tui::{Backend, Buffer, Color, Event, Letter, Quad, Slot},
    util::Result,
};

#[derive(Default)]
pub struct FrameBuffer {
    buffer: Buffer<Slot>,
}

impl FrameBuffer {
    pub fn map_event(&mut self, event: Option<Event>) -> Option<Event> {
        if let Some(event) = event {
            match event {
                Event::Resize { w, h } => {
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

    pub fn render<F>(&mut self, rect: Quad, z: i32, renderer: F)
    where
        F: Fn(Quad, &mut Letter),
    {
        for (x, y) in rect.iter(false) {
            if let Some(slot) = self.buffer.get_mut(rect.x + x, rect.y + y) {
                if z >= slot.z {
                    slot.z = z;
                    let quad = Quad::xywh(x, y, rect.w, rect.h);
                    renderer(quad, &mut slot.letter)
                }
            }
        }
    }

    pub fn draw(&self, backend: &mut Box<dyn Backend>) -> Result {
        let mut seq = Sequencer::new(backend);
        let x0 = self.buffer.rect().x;

        seq.backend.bg(seq.bg)?;
        seq.backend.fg(seq.fg)?;
        for (c, x, y) in self.buffer.iter(true) {
            if x == x0 {
                seq.flush()?;
            }
            seq.step(&c.letter, x, y)?;
        }
        seq.flush()?;

        Ok(())
    }

    pub fn clear(&mut self, backend: &mut Box<dyn Backend>, c: Color) -> Result {
        for (c, _, _) in self.buffer.iter_mut(true) {
            *c = Slot::new();
        }
        backend.bg(c)?;
        backend.clear()?;
        backend.flush()?;
        Ok(())
    }

    pub fn resize(&mut self, w: i32, h: i32) -> bool {
        self.buffer.resize(w, h, Slot::new())
    }
}

struct Sequencer<'a> {
    backend: &'a mut Box<dyn Backend>,
    fg: Color,
    bg: Color,
    buf: String,
}

impl<'a> Sequencer<'a> {
    fn new(backend: &'a mut Box<dyn Backend>) -> Self {
        Self {
            backend,
            fg: Color::WHITE,
            bg: Color::BLACK,
            buf: String::from(""),
        }
    }

    fn step(&mut self, letter: &Letter, x: i32, y: i32) -> Result {
        if letter.hot() {
            self.set_bg(*letter.bg)?;
            self.set_fg(*letter.fg)?;
            self.place(x, y, *letter.c)?;
        } else {
            self.flush()?;
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
