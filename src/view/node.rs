use super::{Attribute, Compose, Composer, Render, Renderer};
use crate::{
    tui::Context,
    util::{SharedPtr, WeakPtr},
};

pub enum Node {
    Compose(Composer),
    Render(Renderer),
}

impl Node {
    pub(super) fn from_composer(widget: Composer) -> Node {
        Self::Compose(widget)
    }

    pub(super) fn from_renderer(widget: Renderer) -> Node {
        Self::Render(widget)
    }

    pub fn root<T: Compose>(view: T) -> (Self, NodeRef<T>) {
        let composer = Composer::new(view);
        let node_ref = composer.get_ref();
        let node = Node::from_composer(composer);

        (node, node_ref)
    }

    pub fn resize(&mut self, w: i32, h: i32) {
        if let Self::Compose(com) = self {
            com.resize(w, h);
        }
    }

    pub fn compose(&mut self) {
        if let Self::Compose(node) = self {
            node.compose();
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        match self {
            Self::Compose(node) => node.render(ctx),
            Self::Render(node) => node.render(ctx),
        }
    }
}

pub struct NodeList {
    pub list: Vec<Node>,
}

impl NodeList {
    pub(super) fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn register_composer<T>(&mut self, widget: T) -> NodeRef<T>
    where
        T: Compose,
    {
        let composer = Composer::new(widget);
        let ptr = composer.get_ref();

        self.list.push(Node::from_composer(composer));
        ptr
    }

    pub fn register_renderer<T>(&mut self, widget: T) -> NodeRef<T>
    where
        T: Render,
    {
        let renderer = Renderer::new(widget);
        let ptr = renderer.get_ref();

        self.list.push(Node::from_renderer(renderer));
        ptr
    }
}

pub struct NodeRef<T: 'static> {
    widget: WeakPtr<T>,
    attr: WeakPtr<Attribute>,
}

impl<T: 'static> NodeRef<T> {
    pub(super) fn new(widget: &SharedPtr<T>, attr: &SharedPtr<Attribute>) -> Self {
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
