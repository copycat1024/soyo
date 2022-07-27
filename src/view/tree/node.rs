use super::Host;
use crate::{
    tui::Context,
    util::{SharedPtr, WeakPtr},
    view::{Compose, Composer, Frame, Render, Renderer},
};

pub struct Node {
    widget: WeakPtr<dyn Host>,
}

impl Node {
    pub(super) fn from_composer<T: Compose>(widget: &Composer<T>) -> Node {
        let widget: SharedPtr<dyn Host> = widget.ptr.clone();
        Self {
            widget: widget.downgrade(),
        }
    }

    pub(super) fn from_renderer<T: Render>(widget: &Renderer<T>) -> Node {
        let widget: SharedPtr<dyn Host> = widget.ptr.clone();
        Self {
            widget: widget.downgrade(),
        }
    }

    pub fn root<T: Compose>(widget: T) -> (Self, Composer<T>) {
        let composer = Composer::new(widget);
        let node = Node::from_composer(&composer);

        (node, composer)
    }

    pub fn layout(&mut self, frame: Frame) {
        if let Some(mut node) = self.widget.upgrade() {
            node.borrow_mut().layout(frame);
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        if let Some(node) = self.widget.upgrade() {
            node.borrow().render(ctx)
        }
    }

    pub fn tick(&mut self, delta: u64) -> bool {
        if let Some(mut node) = self.widget.upgrade() {
            node.borrow_mut().tick(delta)
        } else {
            false
        }
    }
}
