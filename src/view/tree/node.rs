use super::Widget;
use crate::{
    tui::Context,
    util::{SharedPtr, WeakPtr},
    view::{Compose, Composer, Render, Renderer},
};

pub struct Node {
    widget: WeakPtr<dyn Widget>,
}

impl Node {
    pub(super) fn from_composer<T: Compose>(widget: &Composer<T>) -> Node {
        let widget: SharedPtr<dyn Widget> = widget.ptr.clone();
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

    pub fn root<T: Compose>(widget: T) -> (Self, Composer<T>) {
        let composer = Composer::new(widget);
        let node = Node::from_composer(&composer);

        (node, composer)
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
