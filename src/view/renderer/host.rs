use super::Render;
use crate::{
    tui::Context,
    view::{Attribute, Frame, Host},
};
use std::time::Duration;

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
            widget,
            attr: Attribute::default(),
        }
    }
}

impl<T: Render> Host for RenderHost<T> {
    fn render(&self, ctx: &mut Context) {
        let frame = self.attr.frame;
        ctx.render(frame.quad(), frame.z_value(), |q, l| {
            self.widget.render(q, l)
        });
    }

    fn layout(&mut self, frame: Frame) -> Frame {
        let frame = self.widget.layout(frame);
        self.attr.frame = frame;
        frame
    }

    fn tick(&mut self, delta: Duration) -> bool {
        self.widget.tick(delta)
    }
}
