use super::{Attribute, NodeList, NodeRef};
use crate::{
    tui::Context,
    util::{SharedPtr, WeakPtr},
};

pub trait Compose: 'static {
    fn register(&mut self, children: &mut NodeList);
    fn compose(&mut self, attr: &Attribute, children: &mut NodeList);
}

pub struct Composer<T>
where
    T: Compose,
{
    widget: SharedPtr<T>,
    attr: SharedPtr<Attribute>,
    children: SharedPtr<NodeList>,
}

impl<T> Composer<T>
where
    T: Compose + 'static,
{
    pub fn new(mut widget: T) -> Self {
        let mut children = NodeList::new();
        widget.register(&mut children);

        Self {
            widget: SharedPtr::new(widget),
            attr: SharedPtr::new(Attribute::default()),
            children: SharedPtr::new(children),
        }
    }

    pub fn get_ref(&self) -> NodeRef<T> {
        NodeRef::new(&self.widget, &self.attr)
    }

    pub fn node(&self) -> ComposerNode {
        let widget: SharedPtr<dyn Compose> = self.widget.clone();
        ComposerNode {
            widget: widget.downgrade(),
            attr: self.attr.downgrade(),
            children: self.children.downgrade(),
        }
    }
}

pub struct ComposerNode {
    widget: WeakPtr<dyn Compose>,
    attr: WeakPtr<Attribute>,
    children: WeakPtr<NodeList>,
}

impl ComposerNode {
    pub fn resize(&mut self, w: i32, h: i32) {
        if let Some(mut attr) = self.attr.upgrade() {
            attr.borrow_mut().resize(w, h);
        }
    }

    pub fn compose(&mut self) {
        let mut widget_ptr = match self.widget.upgrade() {
            Some(ptr) => ptr,
            None => return,
        };
        let mut widget = widget_ptr.borrow_mut();

        if let Some(attr) = self.attr.upgrade()
        && let Some(mut children) = self.children.upgrade() {
            widget.compose(&attr.borrow(), &mut children.borrow_mut());
            for node in children.borrow_mut().list.iter_mut() {
                node.compose();
            }
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        if let Some(mut children) = self.children.upgrade() {
            for node in children.borrow_mut().list.iter() {
                node.render(ctx);
            }
        }
    }
}
