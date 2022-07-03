use crate::{
    log::{log, tag},
    tui::{Backend, Color, Event, Key},
    util::Result,
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

pub struct Vt100<W: Write + 'static> {
    writer: W,
    last_update: Instant,
}

impl<W: Write + 'static> Vt100<W> {
    pub fn new(mut writer: W) -> Self {
        enter(&mut writer).expect("Cannot enter crossterm.");
        Self {
            writer,
            last_update: Instant::now(),
        }
    }
}

impl<W: Write + 'static> Backend for Vt100<W> {
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

    fn print(&mut self, txt: &str) -> Result {
        self.writer.queue(Print(txt))?;
        writeln!(log(tag::BACKEND), "Print('{txt}')");
        Ok(())
    }

    fn gotoxy(&mut self, x: i32, y: i32) -> Result {
        self.writer.queue(MoveTo(x as u16, y as u16))?;
        writeln!(log(tag::BACKEND), "MoveTo({x},{y})");
        Ok(())
    }

    fn fg(&mut self, color: Color) -> Result {
        let color = crossterm::style::Color::AnsiValue(color.0);
        self.writer.queue(SetForegroundColor(color))?;
        Ok(())
    }

    fn bg(&mut self, color: Color) -> Result {
        let color = crossterm::style::Color::AnsiValue(color.0);
        self.writer.queue(SetBackgroundColor(color))?;
        Ok(())
    }

    fn clear(&mut self) -> Result {
        self.writer
            .queue(ResetColor)?
            .queue(Clear(ClearType::All))?;
        Ok(())
    }

    fn flush(&mut self) -> Result {
        self.writer.flush()?;
        Ok(())
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

        KeyCode::Up => Some(Key::UP),
        KeyCode::Down => Some(Key::DOWN),
        KeyCode::Left => Some(Key::LEFT),
        KeyCode::Right => Some(Key::RIGHT),

        _ => None,
    }
}
