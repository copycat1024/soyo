use super::Node;
use crate::view::{Compose, Composer, Render, Renderer};

pub struct NodeList {
    pub list: Vec<Node>,
}

impl NodeList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn register_composer<T: Compose>(&mut self, widget: T) -> Composer<T> {
        let composer = Composer::new(widget);
        self.list.push(Node::from_composer(&composer));
        composer
    }

    pub fn register_renderer<T: Render>(&mut self, widget: &Renderer<T>) {
        self.list.push(Node::from_renderer(&widget));
    }
}
