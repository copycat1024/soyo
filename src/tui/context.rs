use crate::{
    logger::{Client, Server, Tag},
    tui::{backend::Backend, Color, Event, Frame, Letter, Rect},
    util::Result,
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
    event_logger: Client,

    // internal components
    frame: Frame,
    config: Config,
}

impl<B: Backend> Context<B> {
    pub fn compose(mut backend: B, server: Option<&Server>) -> Self {
        let event_logger = if let Some(server) = server {
            // set component logger
            backend.set_logger(server);

            // create event logger
            server.client(Tag::Event)
        } else {
            Client::default()
        };

        Self {
            backend,
            event_logger,
            frame: Frame::default(),
            config: Config::default(),
        }
    }

    pub fn event(&mut self) -> Result<Option<Event>> {
        self.backend
            .event(self.config.event_period, self.config.update_period)
            .map(|event| {
                if let Some(event) = event {
                    writeln!(self.event_logger, "{event:?}").unwrap();
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
