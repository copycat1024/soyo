use crate::{
    tui::{Letter, Quad},
    view::Attribute,
};

pub trait Render: 'static {
    fn arrange(&self, attr: Attribute) -> Attribute {
        attr
    }
    fn render(&self, quad: Quad, letter: &mut Letter);
}
