use super::{Attribute, NodeList, NodeRef, Widget};
use crate::{tui::Context, util::SharedPtr};

pub trait Compose: 'static {
    fn register(&mut self, children: &mut NodeList);
    fn compose(&mut self, attr: &Attribute, children: &mut NodeList);
}

pub struct Composer<T: Compose> {
    widget: SharedPtr<T>,
    attr: SharedPtr<Attribute>,
    children: SharedPtr<NodeList>,
}

impl<T: Compose> Composer<T> {
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
}

impl<T: Compose> Widget for Composer<T> {
    fn render(&self, ctx: &mut Context) {
        for node in self.children.borrow().list.iter() {
            node.render(ctx);
        }
    }

    fn resize(&mut self, w: i32, h: i32) {
        self.attr.borrow_mut().resize(w, h);
    }

    fn compose(&mut self) {
        self.widget
            .borrow_mut()
            .compose(&self.attr.borrow(), &mut self.children.borrow_mut());

        for node in self.children.borrow_mut().list.iter_mut() {
            node.compose();
        }
    }
}
