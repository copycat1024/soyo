use super::{NodeRef, Widget};
use crate::{
    tui::Context,
    util::{SharedPtr, WeakPtr},
    view::{Compose, Composer, Render, Renderer},
};

pub struct Node {
    widget: WeakPtr<dyn Widget>,
}

impl Node {
    pub(super) fn from_composer<T: Compose>(widget: SharedPtr<Composer<T>>) -> Node {
        let widget: SharedPtr<dyn Widget> = widget.clone();
        Self {
            widget: widget.downgrade(),
        }
    }

    pub(super) fn from_renderer<T: Render>(widget: SharedPtr<Renderer<T>>) -> Node {
        let widget: SharedPtr<dyn Widget> = widget.clone();
        Self {
            widget: widget.downgrade(),
        }
    }

    pub fn root<T: Compose>(view: T) -> (Self, NodeRef<T>) {
        let composer = Composer::new(view);
        let node_ref = composer.get_ref();
        let node = Node::from_composer(SharedPtr::new(composer));

        (node, node_ref)
    }

    pub fn resize(&mut self, w: i32, h: i32) {
        if let Some(mut node) = self.widget.upgrade() {
            node.borrow_mut().resize(w, h);
        }
    }

    pub fn compose(&mut self) {
        if let Some(mut node) = self.widget.upgrade() {
            node.borrow_mut().compose();
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        if let Some(node) = self.widget.upgrade() {
            node.borrow().render(ctx)
        }
    }
}
