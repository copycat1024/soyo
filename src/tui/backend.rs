mod ct;

pub use ct::CrosstermBackend;

use crate::util::{LoggerServer, Result};
use crossterm::{event::Event, style::Color};
use std::{io::Write, time::Duration};

pub trait Backend {
    fn event(&mut self, period: Duration) -> Result<Option<Event>>;
    fn print(&mut self, txt: &str) -> Result<&mut Self>;
    fn gotoxy(&mut self, x: i32, y: i32) -> Result<&mut Self>;
    fn fg(&mut self, c: Color) -> Result<&mut Self>;
    fn bg(&mut self, c: Color) -> Result<&mut Self>;
    fn clear(&mut self) -> Result<&mut Self>;
    fn flush(&mut self) -> Result<&mut Self>;
    fn set_logger(&mut self, logger: &LoggerServer);
}
