use crate::tui::Context;

pub trait Widget {
    fn render(&self, ctx: &mut Context);
    fn resize(&mut self, w: i32, h: i32);
    fn compose(&mut self);
}
