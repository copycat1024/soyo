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

pub struct RenderHost<T>
where
    T: Render,
{
    widget: SharedPtr<T>,
    attr: SharedPtr<Attribute>,
}

impl<T: Render> RenderHost<T> {
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

impl<T: Render> Widget for RenderHost<T> {
    fn render(&self, ctx: &mut Context) {
        let frame = self.attr.borrow().frame;
        ctx.render(frame.quad(), frame.z_value(), |q, l| {
            self.widget.borrow().render(q, l)
        });
    }

    fn resize(&mut self, _: i32, _: i32) {}

    fn compose(&mut self) {}
}

pub struct Renderer<T: Render> {
    pub ptr: SharedPtr<RenderHost<T>>,
}

impl<T: Render> Renderer<T> {
    pub fn new(composer: T) -> Self {
        Self {
            ptr: SharedPtr::new(RenderHost::new(composer)),
        }
    }

    pub fn compose<F>(&mut self, callback: F)
    where
        F: Fn(&mut Attribute),
    {
        callback(&mut self.ptr.borrow_mut().attr.borrow_mut())
    }

    pub fn view<F, R>(&mut self, callback: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        callback(&mut self.ptr.borrow_mut().widget.borrow_mut())
    }
}

impl<T: Render + Default> Default for Renderer<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
