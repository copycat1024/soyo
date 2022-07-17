use super::{Node, NodeRef};
use crate::{
    util::SharedPtr,
    view::{Compose, ComposerHost, Render, Renderer},
};

pub struct NodeList {
    pub list: Vec<Node>,
}

impl NodeList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn register_composer<T: Compose>(&mut self, widget: T) -> ComposerHost<T> {
        let composer = ComposerHost::new(widget);
        self.list.push(Node::from_composer(composer.widget.clone()));
        composer
    }

    pub fn register_renderer<T>(&mut self, widget: T) -> NodeRef<T>
    where
        T: Render,
    {
        let renderer = Renderer::new(widget);
        let ptr = renderer.get_ref();

        let renderer = SharedPtr::new(renderer);
        self.list.push(Node::from_renderer(renderer));
        ptr
    }
}
