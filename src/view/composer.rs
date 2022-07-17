use super::{Attribute, NodeList, Widget};
use crate::{tui::Context, util::SharedPtr};

pub trait Compose: 'static {
    fn register(&mut self, children: &mut NodeList);
    fn compose(&mut self, attr: &Attribute, children: &mut NodeList);
}

pub struct ComposeHost<T: Compose> {
    widget: T,
    attr: Attribute,
    children: NodeList,
}

impl<T: Compose> ComposeHost<T> {
    pub fn new(mut widget: T) -> Self {
        let mut children = NodeList::new();
        widget.register(&mut children);

        Self {
            widget: widget,
            attr: Attribute::default(),
            children: children,
        }
    }
}

impl<T: Compose> Widget for ComposeHost<T> {
    fn render(&self, ctx: &mut Context) {
        for node in self.children.list.iter() {
            node.render(ctx);
        }
    }

    fn resize(&mut self, w: i32, h: i32) {
        self.attr.resize(w, h);
    }

    fn compose(&mut self) {
        self.widget.compose(&self.attr, &mut self.children);

        for node in self.children.list.iter_mut() {
            node.compose();
        }
    }
}

pub struct Composer<T: Compose> {
    pub ptr: SharedPtr<ComposeHost<T>>,
}

impl<T: Compose> Composer<T> {
    pub fn new(composer: T) -> Self {
        Self {
            ptr: SharedPtr::new(ComposeHost::new(composer)),
        }
    }

    pub fn call_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut T),
    {
        f(&mut self.ptr.borrow_mut().widget)
    }
}
