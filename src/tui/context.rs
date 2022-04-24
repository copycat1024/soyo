use crate::{
    tui::{backend::Backend, Color, Event, Frame, Letter, Rect},
    util::{LoggerClient, LoggerServer, Result},
};
use std::{io::Write, time::Duration};

#[derive(Clone, Copy)]
struct Config {
    event_period: Duration,
    update_period: Duration,
    clear_bg: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            event_period: Duration::from_millis(10),
            update_period: Duration::from_millis(1000),
            clear_bg: Color::BLACK,
        }
    }
}

pub struct Context<B: Backend> {
    // external components
    backend: B,
    logger: LoggerClient,

    // internal components
    frame: Frame,
    config: Config,
}

impl<B: Backend> Context<B> {
    pub fn compose(mut backend: B, logserver: Option<&LoggerServer>) -> Self {
        let mut frame = Frame::default();
        let mut logger = LoggerClient::default();

        if let Some(logserver) = logserver {
            frame.set_logger(logserver);
            backend.set_logger(logserver);
            logger = logserver.client();
        }

        Self {
            backend,
            logger,
            frame,
            config: Config::default(),
        }
    }

    pub fn event(&mut self) -> Result<Option<Event>> {
        self.backend
            .event(self.config.event_period, self.config.update_period)
            .map(|event| {
                if let Some(event) = event {
                    writeln!(self.logger, "{event:?}").unwrap();
                };
                self.frame.map_event(event)
            })
    }

    pub fn render<F>(&mut self, rect: Rect, z: i32, renderer: F)
    where
        F: Fn(i32, i32, &mut Letter),
    {
        self.frame.render(rect, z, renderer);
    }

    pub fn draw(&mut self) -> Result {
        let Self { frame, backend, .. } = self;
        frame.draw(backend)
    }

    pub fn clear(&mut self) -> Result {
        let Self { frame, backend, .. } = self;
        frame.clear(backend, self.config.clear_bg)
    }
}
