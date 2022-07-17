use crate::tui::Context;

pub trait Widget {
    fn render(&self, ctx: &mut Context);
}
