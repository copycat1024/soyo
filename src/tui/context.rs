use super::{backend::Backend, Frame, Letter, Rect};
use crate::util::{HotRef, Result};
use crossterm::{
    event::{poll, read, Event, KeyEvent},
    style::Color,
};
use std::{cell::Cell, time::Duration};

pub struct Context<B: Backend> {
    backend: B,
    frame: Frame,
    period: Duration,
    clear_bg: Color,
    log: String,
}

impl<B: Backend> Context<B> {
    pub fn event(&mut self) -> Result<Option<Event>> {
        self.backend.event(self.period).map(|option| {
            if let Some(event) = option {
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
            }
        })
    }

    pub fn item(&mut self, x: i32, y: i32, z: i32) -> Option<HotRef<Letter>> {
        self.frame.item(x, y, z)
    }

    pub fn draw(&mut self) -> Result {
        let Self { frame, backend, .. } = self;
        frame.draw(backend)
    }

    pub fn clear(&mut self) -> Result {
        let Self { frame, backend, .. } = self;
        frame.clear(backend, self.clear_bg)
    }

    pub fn leak_log(&self) -> String {
        self.log.clone()
    }
}

impl<B: Backend> Default for Context<B> {
    fn default() -> Self {
        Self {
            backend: B::default(),
            frame: Frame::new(),
            period: Duration::from_millis(100),
            clear_bg: Color::Red,
            log: String::new(),
        }
    }
}
