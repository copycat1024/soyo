use crate::{tui::Context, view::Frame};
use std::time::Duration;

pub trait Host {
    fn render(&self, ctx: &mut Context);
    fn layout(&mut self, attr: Frame) -> Frame;
    fn tick(&mut self, delta: Duration) -> bool;
}
