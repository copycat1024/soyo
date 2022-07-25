use crate::{tui::Context, view::Frame};

pub trait Host {
    fn render(&self, ctx: &mut Context);
    fn layout(&mut self, attr: Frame) -> Frame;
}
