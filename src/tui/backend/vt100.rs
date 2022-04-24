use crate::{
    tui::{Backend, Color, Event, Key},
    util::{LoggerClient, LoggerServer, Result},
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, KeyCode},
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};
use std::{
    io::Write,
    time::{Duration, Instant},
};

pub struct Vt100<W: Write> {
    writer: W,
    logger: LoggerClient,
    last_update: Instant,
}

impl<W: Write> Vt100<W> {
    pub fn new(mut writer: W) -> Self {
        enter(&mut writer).expect("Cannot enter crossterm.");
        Self {
            writer,
            logger: LoggerClient::default(),
            last_update: Instant::now(),
        }
    }
}

impl<W: Write> Backend for Vt100<W> {
    fn event(&mut self, event_period: Duration, update_period: Duration) -> Result<Option<Event>> {
        let event = if poll(event_period)? {
            match read()? {
                crossterm::event::Event::Key(key) => {
                    map_key(key.code).map(|key| Event::Key { key })
                }
                crossterm::event::Event::Resize(w, h) => Some(Event::Resize {
                    w: w as i32,
                    h: h as i32,
                }),
                _ => None,
            }
        } else {
            let now = Instant::now();
            let delta = now.duration_since(self.last_update);
            if delta > update_period {
                self.last_update = now;
                Some(Event::Update { delta })
            } else {
                None
            }
        };

        Ok(event)
    }

    fn print(&mut self, txt: &str) -> Result<&mut Self> {
        self.writer.queue(Print(txt))?;
        writeln!(self.logger, "Print('{txt}')").unwrap();
        Ok(self)
    }

    fn gotoxy(&mut self, x: i32, y: i32) -> Result<&mut Self> {
        self.writer.queue(MoveTo(x as u16, y as u16))?;
        writeln!(self.logger, "MoveTo({x},{y})").unwrap();
        Ok(self)
    }

    fn fg(&mut self, color: Color) -> Result<&mut Self> {
        let color = crossterm::style::Color::AnsiValue(color.0);
        self.writer.queue(SetForegroundColor(color))?;
        Ok(self)
    }

    fn bg(&mut self, color: Color) -> Result<&mut Self> {
        let color = crossterm::style::Color::AnsiValue(color.0);
        self.writer.queue(SetBackgroundColor(color))?;
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

impl<W: Write> Drop for Vt100<W> {
    fn drop(&mut self) {
        leave(&mut self.writer).expect("Cannot leave crossterm.");
    }
}

fn enter<W: Write>(writer: &mut W) -> Result {
    enable_raw_mode()?;
    writer
        .execute(DisableLineWrap)?
        .execute(EnterAlternateScreen)?
        .execute(Hide)?;
    Ok(())
}

fn leave<W: Write>(writer: &mut W) -> Result {
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

fn map_key(key: KeyCode) -> Option<Key> {
    match key {
        KeyCode::Char(c) => Some(Key(c)),
        KeyCode::Enter => Some(Key::ENTER),
        KeyCode::Esc => Some(Key::ESC),
        _ => None,
    }
}
