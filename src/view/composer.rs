use super::{Attribute, NodeList, NodeRef};
use crate::{tui::Context, util::SharedPtr};
use std::{marker::Unsize, ops::CoerceUnsized};

pub trait Compose: 'static {
    fn register(&mut self, children: &mut NodeList);
    fn compose(&mut self, attr: &Attribute, children: &mut NodeList);
}

pub struct Composer<T = dyn Compose>
where
    T: Compose + ?Sized,
{
    widget: SharedPtr<T>,
    attr: SharedPtr<Attribute>,
    children: NodeList,
}

impl<T> Composer<T>
where
    T: Compose,
{
    pub fn new(mut widget: T) -> Self {
        let mut children = NodeList::new();
        widget.register(&mut children);

        Self {
            widget: SharedPtr::new(widget),
            attr: SharedPtr::new(Attribute::default()),
            children,
        }
    }

    pub fn get_ref(&self) -> NodeRef<T> {
        NodeRef::new(&self.widget, &self.attr)
    }
}

impl<T> Composer<T>
where
    T: Compose + ?Sized,
{
    pub fn resize(&mut self, w: i32, h: i32) {
        self.attr.borrow_mut().resize(w, h);
    }

    pub fn compose(&mut self) {
        let Self {
            widget,
            attr,
            children,
        } = self;

        widget.borrow_mut().compose(&attr.borrow(), children);
        for node in children.list.iter_mut() {
            node.compose();
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        for node in self.children.list.iter() {
            node.render(ctx);
        }
    }
}

impl<T, U> CoerceUnsized<Composer<U>> for Composer<T>
where
    T: Compose + Unsize<U> + ?Sized,
    U: Compose + ?Sized,
{
}
