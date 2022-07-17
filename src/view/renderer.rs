use super::{Attribute, NodeRef, Widget};
use crate::{
    tui::{Context, Letter, Quad},
    util::SharedPtr,
};

pub trait Render: 'static {
    fn arrange(&self, attr: Attribute) -> Attribute {
        attr
    }
    fn render(&self, quad: Quad, letter: &mut Letter);
}

pub struct Renderer<T>
where
    T: Render,
{
    widget: SharedPtr<T>,
    attr: SharedPtr<Attribute>,
}

impl<T: Render> Renderer<T> {
    pub fn new(widget: T) -> Self {
        Self {
            widget: SharedPtr::new(widget),
            attr: SharedPtr::new(Attribute::default()),
        }
    }

    pub fn get_ref(&self) -> NodeRef<T> {
        NodeRef::new(&self.widget, &self.attr)
    }
}

impl<T: Render> Widget for Renderer<T> {
    fn render(&self, ctx: &mut Context) {
        let frame = self.attr.borrow().frame;
        ctx.render(frame.quad(), frame.z_value(), |q, l| {
            self.widget.borrow().render(q, l)
        });
    }

    fn resize(&mut self, _: i32, _: i32) {}

    fn compose(&mut self) {}
}
