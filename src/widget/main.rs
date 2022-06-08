use super::{Composer, Layer, Render};
use crate::tui::Context;

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

    pub fn render(&self, ctx: &mut Context, layer: Layer) {
        let layer = self.composer.compose(layer);
        ctx.render(layer.quad(), layer.z_value(), |q, l| {
            self.widget.render(q, l)
        });
    }
}
