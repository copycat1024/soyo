use super::Flow;
use crate::{
    tui::Context,
    util::Result,
    view::{Compose, Node, NodeRef},
};

pub struct View<T: 'static> {
    root: Node,
    root_ref: NodeRef<T>,
}

impl<T: 'static + Compose> View<T> {
    pub fn new(node: T) -> Self {
        let (root, root_ref) = Node::root(node);
        Self { root, root_ref }
    }

    pub fn resize(&mut self, w: i32, h: i32) {
        self.root.resize(w, h);
        self.root.compose();
    }

    pub fn draw(&self, ctx: &mut Context, flow: &mut Flow) -> Result {
        if flow.clear {
            flow.clear = false;
            ctx.clear()?;
        }

        if flow.draw {
            flow.draw = false;
            self.root.render(ctx);
            ctx.draw()?;
        }

        Ok(())
    }

    pub fn node(&self) -> &NodeRef<T> {
        &self.root_ref
    }

    pub fn node_mut(&mut self) -> &mut NodeRef<T> {
        &mut self.root_ref
    }
}
