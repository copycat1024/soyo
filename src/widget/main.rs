use super::{Composer, Render};
use crate::tui::{Context, Quad};

pub struct Widget<T: Render> {
    pub widget: T,
    pub composer: Composer,
}

impl<T: Render> Widget<T> {
    pub fn new(widget: T) -> Self {
        Self {
            widget,
            composer: Composer::default(),
        }
    }

    pub fn render(&self, ctx: &mut Context, quad: Quad) {
        let (quad, z) = self.composer.compose(quad, 0);
        ctx.render(quad, z, |q, l| self.widget.render(q, l));
    }
}
