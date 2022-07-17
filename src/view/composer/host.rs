use super::Compose;
use crate::{
    tui::Context,
    view::{Attribute, NodeList, Widget},
};

pub struct ComposeHost<T: Compose> {
    pub widget: T,
    pub attr: Attribute,
    pub children: NodeList,
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
