use super::Backend;
use crate::util::{LoggerClient, LoggerServer, Result};
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
    io::Write,
    rc::{Rc, Weak},
    time::Duration,
};

pub struct CrosstermBackend<W: Write> {
    writer: W,
    logger: LoggerClient,
}

impl<W: Write> CrosstermBackend<W> {
    pub fn new(mut writer: W) -> Self {
        enter(&mut writer).expect("Cannot enter crossterm.");
        Self {
            writer,
            logger: LoggerClient::new(),
        }
    }
}

impl<W: Write> Backend for CrosstermBackend<W> {
    fn event(&mut self, period: Duration) -> Result<Option<Event>> {
        if poll(period)? {
            read().map(Some).map_err(|err| err.into())
        } else {
            Ok(None)
        }
    }

    fn print(&mut self, txt: &str) -> Result<&mut Self> {
        self.writer.queue(Print(txt))?;
        writeln!(self.logger, "Print('{txt}')").unwrap();
        Ok(self)
    }

    fn gotoxy(&mut self, x: i32, y: i32) -> Result<&mut Self> {
        self.writer.queue(MoveTo(x as u16, y as u16))?;
        writeln!(self.logger, "MoveTo('{x},{y}')").unwrap();
        Ok(self)
    }

    fn fg(&mut self, c: Color) -> Result<&mut Self> {
        self.writer.queue(SetForegroundColor(c))?;
        Ok(self)
    }

    fn bg(&mut self, c: Color) -> Result<&mut Self> {
        self.writer.queue(SetBackgroundColor(c))?;
        Ok(self)
    }

    fn clear(&mut self) -> Result<&mut Self> {
        self.writer
            .queue(ResetColor)?
            .queue(Clear(ClearType::All))?;
        Ok(self)
    }

    fn flush(&mut self) -> Result<&mut Self> {
        self.writer.flush()?;
        Ok(self)
    }

    fn set_logger(&mut self, logger: &LoggerServer) {
        self.logger = logger.client();
    }
}

impl<W: Write> Drop for CrosstermBackend<W> {
    fn drop(&mut self) {
        leave(&mut self.writer).expect("Cannot leave crossterm.");
    }
}

pub fn enter<W: Write>(writer: &mut W) -> Result {
    enable_raw_mode()?;
    writer
        .execute(DisableLineWrap)?
        .execute(EnterAlternateScreen)?
        .execute(Hide)?;
    Ok(())
}

pub fn leave<W: Write>(writer: &mut W) -> Result {
    disable_raw_mode()?;
    writer
        .execute(LeaveAlternateScreen)?
        .execute(ResetColor)?
        .execute(EnableLineWrap)?
        .execute(Clear(ClearType::All))?
        .execute(MoveTo(0, 0))?
        .execute(Show)?;
    Ok(())
}
