use crate::view::{Attribute, NodeList};

pub trait Compose: 'static {
    fn register(&mut self, children: &mut NodeList);
    fn compose(&mut self, attr: &Attribute, children: &mut NodeList);
}
