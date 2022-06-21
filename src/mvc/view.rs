use crate::tui::Context;

pub trait View: Default + 'static {
    fn setup(&mut self);
    fn resize(&mut self, w: i32, h: i32);
    fn render(&self, ctx: &mut Context);
}
