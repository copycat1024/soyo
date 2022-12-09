use super::Flow;
use crate::{
    tui::Context,
    util::Result,
    view::{Compose, Composer, Frame, Node},
};

pub struct View<T: Compose> {
    root: Node,
    root_ref: Composer<T>,
    screen: Frame,
}

impl<T: 'static + Compose> View<T> {
    pub fn new(node: T) -> Self {
        let (root, root_ref) = Node::root(node);
        Self {
            root,
            root_ref,
            screen: Frame::screen(0, 0),
        }
    }

    pub fn resize(&mut self, w: i32, h: i32, ctx: &mut Context, flow: &mut Flow) -> Result {
        flow.draw = true;
        self.screen = Frame::screen(w, h);
        ctx.clear()
    }

    pub fn tick(&mut self, delta: u64, flow: &mut Flow) {
        if self.root.tick(delta) {
            flow.draw = true;
        }
    }

    pub fn compose(&mut self) {
        self.root.layout(self.screen);
    }

    pub fn draw(&mut self, ctx: &mut Context, flow: &mut Flow) -> Result {
        if flow.clear {
            flow.clear = false;
            ctx.clear()?;
        }

        flow.draw = false;
        self.root.render(ctx);
        ctx.draw()?;

        Ok(())
    }

    pub fn node(&self) -> &Composer<T> {
        &self.root_ref
    }

    pub fn node_mut(&mut self) -> &mut Composer<T> {
        &mut self.root_ref
    }
}
