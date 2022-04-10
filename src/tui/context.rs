use super::{backend::Backend, Frame, Letter, Rect};
use crate::util::{HotRef, Result};
use crossterm::{
    event::{poll, read, Event, KeyEvent},
    style::Color,
};
use std::{cell::Cell, time::Duration};

#[derive(Clone, Copy)]
struct Config {
    period: Duration,
    clear_bg: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            period: Duration::from_millis(100),
            clear_bg: Color::Red,
        }
    }
}

pub struct Context<'a, B: Backend> {
    // external components
    backend: &'a mut B,

    // internal components
    frame: Frame,
    config: Config,

    log: String,
    count: u64,
}

impl<'a, B: Backend> Context<'a, B> {
    pub fn compose(backend: &'a mut B) -> Self {
        Self {
            backend,
            frame: Frame::default(),
            config: Config::default(),
            log: String::new(),
            count: 0,
        }
    }

    pub fn event(&mut self) -> Result<Option<Event>> {
        self.backend.event(self.config.period).map(|event| {
            match event {
                Some(event) => {
                    let count = self.count;
                    let logger = self.backend.logger();
                    if count > 0 {
                        writeln!(logger, "None({count})");
                    }
                    writeln!(logger, "{event:?}");
                }
                None => {
                    self.count += 1;
                }
            }
            self.frame.map_event(event)
        })
    }

    pub fn item(&mut self, x: i32, y: i32, z: i32) -> Option<HotRef<Letter>> {
        self.frame.item(x, y, z)
    }

    pub fn draw(&mut self) -> Result {
        let Self { frame, backend, .. } = self;
        frame.draw(*backend)
    }

    pub fn clear(&mut self) -> Result {
        let Self { frame, backend, .. } = self;
        frame.clear(*backend, self.config.clear_bg)
    }

    pub fn leak_log(&self) -> String {
        self.log.clone()
    }
}
