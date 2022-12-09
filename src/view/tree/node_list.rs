use super::Node;
use crate::view::{Compose, Composer, Render, Renderer};

#[derive(Default)]
pub struct NodeList {
    pub list: Vec<Node>,
}

impl NodeList {
    pub fn register_composer<T: Compose>(&mut self, widget: &Composer<T>) {
        self.list.push(Node::from_composer(widget));
    }

    pub fn register_renderer<T: Render>(&mut self, widget: &Renderer<T>) {
        self.list.push(Node::from_renderer(widget));
    }
}
