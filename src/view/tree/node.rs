use super::NodeRef;
use crate::{
    tui::Context,
    view::{Compose, Composer, ComposerNode, Renderer},
};

pub enum Node {
    Compose(ComposerNode),
    Render(Renderer),
}

impl Node {
    pub(super) fn from_composer<T: Compose>(widget: Composer<T>) -> Node {
        Self::Compose(widget.node())
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
