mod vt100;

pub use vt100::Vt100;

use crate::{
    tui::{Color, Event},
    util::Result,
};
use std::time::Duration;

pub trait Backend: 'static {
    fn event(&mut self, event_period: Duration, update_period: Duration) -> Result<Option<Event>>;
    fn print(&mut self, txt: &str) -> Result;
    fn gotoxy(&mut self, x: i32, y: i32) -> Result;
    fn fg(&mut self, c: Color) -> Result;
    fn bg(&mut self, c: Color) -> Result;
    fn clear(&mut self) -> Result;
    fn flush(&mut self) -> Result;
}
