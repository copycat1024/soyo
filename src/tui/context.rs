use super::{Frame, Letter, Rect};
use crate::util::{HotRef, Result};
use crossterm::{
    event::{poll, read, Event, KeyEvent},
    style::Color,
};
use std::{cell::Cell, time::Duration};

pub struct Context {
    frame: Frame,
    period: Duration,
    clear_bg: Color,
}

impl Context {
    pub fn event(&mut self) -> Result<Option<Event>> {
        Ok(if poll(self.period)? {
            let event = read()?;

            match event {
                Event::Resize(w, h) => {
                    if self.frame.resize(w as i32, h as i32) {
                        Some(event)
                    } else {
                        None
                    }
                }
                _ => Some(event),
            }
        } else {
            None
        })
    }

    pub fn item(&mut self, x: i32, y: i32, z: i32) -> Option<HotRef<Letter>> {
        self.frame.item(x, y, z)
    }

    pub fn draw(&self) -> Result {
        use std::io::{stdout, Write};

        self.frame.draw()?;
        stdout().flush()?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result {
        use std::io::{stdout, Write};

        self.frame.clear(self.clear_bg)?;
        stdout().flush()?;

        Ok(())
    }

    pub fn log<F>(f: F) -> Result
    where
        F: Fn() -> Result,
    {
        leave()?;
        f()?;
        enter()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        leave().expect("Cannot leave silk");
    }
}

impl Default for Context {
    fn default() -> Self {
        enter().expect("Cannot enter silk");
        Self {
            frame: Frame::new(),
            period: Duration::from_millis(100),
            clear_bg: Color::Red,
        }
    }
}

pub fn enter() -> Result {
    use crossterm::{cursor, terminal, ExecutableCommand};
    use std::io::stdout;

    terminal::enable_raw_mode()?;
    stdout()
        .execute(terminal::DisableLineWrap)?
        .execute(terminal::EnterAlternateScreen)?
        .execute(cursor::Hide)?;

    Ok(())
}

pub fn leave() -> Result {
    use crossterm::{cursor, terminal, ExecutableCommand};
    use std::io::stdout;

    stdout()
        .execute(terminal::EnableLineWrap)?
        .execute(terminal::LeaveAlternateScreen)?
        .execute(cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
