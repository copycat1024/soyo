use super::Render;
use crate::{
    tui::Context,
    view::{Attribute, Widget},
};

pub struct RenderHost<T>
where
    T: Render,
{
    pub widget: T,
    pub attr: Attribute,
}

impl<T: Render> RenderHost<T> {
    pub fn new(widget: T) -> Self {
        Self {
            widget: widget,
            attr: Attribute::default(),
        }
    }
}

impl<T: Render> Widget for RenderHost<T> {
    fn render(&self, ctx: &mut Context) {
        let frame = self.attr.frame;
        ctx.render(frame.quad(), frame.z_value(), |q, l| {
            self.widget.render(q, l)
        });
    }

    fn resize(&mut self, _: i32, _: i32) {}

    fn compose(&mut self) {}
}
