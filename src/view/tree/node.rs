use super::{NodeRef, Widget};
use crate::{
    tui::Context,
    util::{SharedPtr, WeakPtr},
    view::{Compose, Composer, ComposerNode, Render, Renderer},
};

pub enum Node {
    Compose(ComposerNode),
    Render(WeakPtr<dyn Widget>),
}

impl Node {
    pub(super) fn from_composer<T: Compose>(widget: Composer<T>) -> Node {
        Self::Compose(widget.node())
    }

    pub(super) fn from_renderer<T: Render>(widget: SharedPtr<Renderer<T>>) -> Node {
        let widget: SharedPtr<dyn Widget> = widget.clone();
        Self::Render(widget.downgrade())
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
            Self::Render(node) => {
                if let Some(node) = node.upgrade() {
                    node.borrow().render(ctx)
                }
            }
        }
    }
}
