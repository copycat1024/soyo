use super::{Node, NodeRef};
use crate::{
    util::SharedPtr,
    view::{Compose, Composer, Render, Renderer},
};

pub struct NodeList {
    pub list: Vec<Node>,
}

impl NodeList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn register_composer<T>(&mut self, widget: T) -> NodeRef<T>
    where
        T: Compose,
    {
        let composer = Composer::new(widget);
        let ptr = composer.get_ref();

        self.list
            .push(Node::from_composer(SharedPtr::new(composer)));
        ptr
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
