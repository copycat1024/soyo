use crate::{
    util::{SharedPtr, WeakPtr},
    view::Attribute,
};

pub struct NodeRef<T: 'static> {
    widget: WeakPtr<T>,
    attr: WeakPtr<Attribute>,
}

impl<T: 'static> NodeRef<T> {
    pub fn new(widget: &SharedPtr<T>, attr: &SharedPtr<Attribute>) -> Self {
        Self {
            widget: widget.downgrade(),
            attr: attr.downgrade(),
        }
    }

    pub fn compose<F>(&mut self, callback: F)
    where
        F: Fn(&mut Attribute),
    {
        if let Some(mut attr) = self.attr.upgrade() {
            callback(&mut attr.borrow_mut())
        }
    }

    pub fn view<F, R>(&mut self, callback: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        self.widget.update(callback)
    }

    pub fn call_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut T),
    {
        if let Some(mut widget) = self.widget.upgrade() {
            let mut widget = widget.borrow_mut();
            f(&mut widget)
        }
    }
}

impl<T: 'static> Default for NodeRef<T> {
    fn default() -> Self {
        Self {
            widget: WeakPtr::default(),
            attr: WeakPtr::default(),
        }
    }
}
