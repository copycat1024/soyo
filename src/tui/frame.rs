use crate::{
    logger::{Client, Server},
    tui::{Backend, Buffer, Color, Event, Letter, Rect, Slot},
    util::Result,
};

#[derive(Default)]
pub struct Frame {
    buffer: Buffer<Slot>,
    logger: Client,
}

impl Frame {
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

    pub fn render<F>(&mut self, rect: Rect, z: i32, renderer: F)
    where
        F: Fn(i32, i32, &mut Letter),
    {
        for (x, y) in rect.iter(false) {
            if let Some(slot) = self.buffer.get_mut(rect.x + x, rect.y + y) {
                if z > slot.z {
                    slot.z = z;
                    renderer(x, y, &mut slot.letter)
                }
            }
        }
    }

    pub fn draw<B: Backend>(&self, backend: &mut B) -> Result {
        let mut seq = Sequencer::new(backend);
        let x0 = self.buffer.rect().x;

        for (c, x, y) in self.buffer.iter(true) {
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

    pub fn resize(&mut self, w: i32, h: i32) -> bool {
        self.buffer.resize(w, h, Slot::new())
    }

    pub fn set_logger(&mut self, logger: &Server) {
        self.logger = logger.client(1);
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
