use crate::{
    log::{log, Tag},
    tui::{backend::Backend, Color, Event, Frame, Letter, Quad},
    util::Result,
};
use std::time::Duration;

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
            update_period: Duration::from_millis(100),
            clear_bg: Color::BLACK,
        }
    }
}

pub struct Context {
    // external components
    backend: Box<dyn Backend>,

    // internal components
    frame: Frame,
    config: Config,
    w: i32,
    h: i32,
}

impl Context {
    pub fn new<B: Backend>(backend: B) -> Self {
        Self {
            backend: Box::new(backend),
            frame: Frame::default(),
            config: Config::default(),
            w: 0,
            h: 0,
        }
    }

    pub fn event(&mut self) -> Result<Option<Event>> {
        self.backend
            .event(self.config.event_period, self.config.update_period)
            .map(|event| {
                if let Some(event) = event {
                    writeln!(log(Tag::Event), "{event:?}");
                    if let Event::Resize { w, h } = event {
                        self.w = w;
                        self.h = h;
                    }
                };
                self.frame.map_event(event)
            })
    }

    pub fn render<F>(&mut self, rect: Quad, z: i32, renderer: F)
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

    pub fn size(&self) -> (i32, i32) {
        (self.w, self.h)
    }
}
