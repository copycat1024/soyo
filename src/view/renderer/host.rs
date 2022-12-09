use super::Render;
use crate::{
    tui::Context,
    view::{Attribute, Frame, Host},
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
            widget,
            attr: Attribute::default(),
        }
    }
}

impl<T: Render> Host for RenderHost<T> {
    fn render(&self, ctx: &mut Context) {
        let frame = self.attr.frame;
        ctx.render(frame.quad(), frame.z_value(), |q, l| {
            *l.fg = self.attr.fg;
            *l.bg = self.attr.bg;
            self.widget.render(q, l)
        });
    }

    fn layout(&mut self, frame: Frame) -> Frame {
        self.attr.frame = frame;
        self.widget.layout(&mut self.attr.frame);
        self.attr.frame
    }

    fn tick(&mut self, delta: u64) -> bool {
        self.widget.tick(delta)
    }
}
