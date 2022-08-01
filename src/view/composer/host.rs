use super::Compose;
use crate::{
    tui::Context,
    view::{Attribute, Frame, Host, NodeList},
};

pub struct ComposeHost<T: Compose> {
    pub widget: T,
    pub attr: Attribute,
    pub children: NodeList,
}

impl<T: Compose> ComposeHost<T> {
    pub fn new(mut widget: T) -> Self {
        let mut children = NodeList::default();
        widget.register(&mut children);

        Self {
            widget,
            attr: Attribute::default(),
            children,
        }
    }
}

impl<T: Compose> Host for ComposeHost<T> {
    fn render(&self, ctx: &mut Context) {
        for node in self.children.list.iter() {
            node.render(ctx);
        }
    }

    fn layout(&mut self, frame: Frame) -> Frame {
        self.attr.frame = frame;
        self.widget.layout(&mut self.attr.frame);
        self.attr.frame
    }

    fn tick(&mut self, delta: u64) -> bool {
        let mut draw = self.widget.tick(delta);
        for node in self.children.list.iter_mut() {
            draw |= node.tick(delta);
        }
        draw
    }
}
