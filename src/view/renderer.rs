use super::{Attribute, NodeRef};
use crate::{
    tui::{Context, Letter, Quad},
    util::SharedPtr,
};
use std::{marker::Unsize, ops::CoerceUnsized};

pub trait Render: 'static {
    fn arrange(&self, attr: Attribute) -> Attribute {
        attr
    }
    fn render(&self, quad: Quad, letter: &mut Letter);
}

pub struct Renderer<T = dyn Render>
where
    T: Render + ?Sized,
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

impl<T> Renderer<T>
where
    T: Render + ?Sized,
{
    pub fn render(&self, ctx: &mut Context) {
        let frame = self.attr.borrow().frame;
        ctx.render(frame.quad(), frame.z_value(), |q, l| {
            self.widget.borrow().render(q, l)
        });
    }
}

impl<T, U> CoerceUnsized<Renderer<U>> for Renderer<T>
where
    T: Render + Unsize<U> + ?Sized,
    U: Render + ?Sized,
{
}
