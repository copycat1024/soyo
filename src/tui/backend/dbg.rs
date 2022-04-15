use super::Backend;
use crate::util::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};
use std::{
    io::{stdout, Write},
    time::Duration,
};

pub struct DebugBackend {
    first: bool,
}

impl Default for DebugBackend {
    fn default() -> Self {
        Self { first: true }
    }
}

impl Backend for DebugBackend {
    fn event(&mut self, period: Duration) -> Result<Option<Event>> {
        let event = if self.first {
            self.first = false;
            let (w, h) = size()?;
            Event::Resize(w, h)
        } else {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
            })
        };

        Ok(Some(event))
    }

    fn print(&mut self, txt: &str) -> Result<&mut Self> {
        println!("print({txt})");
        Ok(self)
    }

    fn gotoxy(&mut self, x: i32, y: i32) -> Result<&mut Self> {
        println!("gotoxy({x},{y})");
        Ok(self)
    }

    fn fg(&mut self, c: Color) -> Result<&mut Self> {
        Ok(self)
    }

    fn bg(&mut self, c: Color) -> Result<&mut Self> {
        Ok(self)
    }

    fn clear(&mut self) -> Result<&mut Self> {
        Ok(self)
    }

    fn flush(&mut self) -> Result<&mut Self> {
        println!("flush()");
        Ok(self)
    }
}
