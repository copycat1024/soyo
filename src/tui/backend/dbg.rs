use super::Backend;
use crate::util::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::Event,
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

#[derive(Default)]
pub struct DebugBackend {
    cmd: String,
}

impl Backend for DebugBackend {
    fn event(&mut self, period: Duration) -> Result<Option<Event>> {
        Ok(None)
    }

    fn print(&mut self, txt: &str) -> Result<&mut Self> {
        Ok(self)
    }

    fn gotoxy(&mut self, x: i32, y: i32) -> Result<&mut Self> {
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
        Ok(self)
    }
}
