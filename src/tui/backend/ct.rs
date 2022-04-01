use super::Backend;
use crate::util::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};
use std::{
    io::{stdout, Write},
    time::Duration,
};

pub struct CrosstermBackend {}

impl Backend for CrosstermBackend {
    fn event(&mut self, period: Duration) -> Result<Option<Event>> {
        if poll(period)? {
            read().map(Some).map_err(|err| err.into())
        } else {
            Ok(None)
        }
    }

    fn print(&mut self, txt: &str) -> Result<&mut Self> {
        stdout().queue(Print(txt))?;

        Ok(self)
    }

    fn gotoxy(&mut self, x: i32, y: i32) -> Result<&mut Self> {
        stdout().queue(MoveTo(x as u16, y as u16))?;

        Ok(self)
    }

    fn fg(&mut self, c: Color) -> Result<&mut Self> {
        stdout().queue(SetForegroundColor(c))?;

        Ok(self)
    }

    fn bg(&mut self, c: Color) -> Result<&mut Self> {
        stdout().queue(SetBackgroundColor(c))?;

        Ok(self)
    }

    fn clear(&mut self) -> Result<&mut Self> {
        stdout().queue(ResetColor)?.queue(Clear(ClearType::All))?;

        Ok(self)
    }

    fn flush(&mut self) -> Result<&mut Self> {
        stdout().flush()?;

        Ok(self)
    }
}

impl Default for CrosstermBackend {
    fn default() -> Self {
        enter().expect("Cannot enter crossterm.");

        Self {}
    }
}

impl Drop for CrosstermBackend {
    fn drop(&mut self) {
        leave().expect("Cannot leave crossterm.");
    }
}

pub fn enter() -> Result {
    enable_raw_mode()?;
    stdout()
        .execute(DisableLineWrap)?
        .execute(EnterAlternateScreen)?
        .execute(Hide)?;
    Ok(())
}

pub fn leave() -> Result {
    disable_raw_mode()?;
    stdout()
        .execute(LeaveAlternateScreen)?
        .execute(ResetColor)?
        .execute(EnableLineWrap)?
        .execute(Clear(ClearType::All))?
        .execute(MoveTo(0, 0))?
        .execute(Show)?;
    Ok(())
}
